use std::io::Write;
use std::process::{Command, Stdio};

fn test_this_one(
    args: &[&str],
    prompt: &str,
    input: &[&str],
    expected_output: &[&str],
) -> anyhow::Result<()> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_computergeneration"))
        .arg(prompt)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().unwrap();
    for word in input {
        child_stdin.write_all(word.as_bytes())?;
        child_stdin.write_all(b"\n")?;
    }
    drop(child_stdin);

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let output_lines: Vec<&str> = stdout.lines().collect();

    assert_eq!(output_lines, expected_output);

    Ok(())
}

#[test]
fn basic_examples() -> anyhow::Result<()> {
    test_this_one(
        &[],
        "hello",
        &["hello, world", "hi", "hello my dude"],
        &["hello, world", "hello my dude"],
    )?;

    test_this_one(&[], "", &["hi", "hey", "sup"], &["hi", "hey", "sup"])?;

    Ok(())
}

#[test]
fn case_insensitive() -> anyhow::Result<()> {
    test_this_one(
        &["--case-insensitive"],
        "HEL",
        &["hello", "Hello", "HELLOO", "heb"],
        &["hello", "Hello", "HELLOO"],
    )?;

    Ok(())
}
