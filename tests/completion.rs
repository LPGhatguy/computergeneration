use std::io::Write;
use std::process::{Command, Stdio};

struct Test<'a> {
    args: &'a [&'a str],
    pattern: &'a str,
    input: &'a [&'a str],
    expected: &'a [&'a str],
}

impl Test<'_> {
    fn run(self) -> anyhow::Result<()> {
        let mut child = Command::new(env!("CARGO_BIN_EXE_computergeneration"))
            .arg(self.pattern)
            .args(self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let child_stdin = child.stdin.as_mut().unwrap();
        for word in self.input {
            child_stdin.write_all(word.as_bytes())?;
            child_stdin.write_all(b"\n")?;
        }
        drop(child_stdin);

        let output = child.wait_with_output()?;

        let status = output.status;
        if !status.success() {
            panic!("Process exited with status code {:?}", status.code());
        }

        let stdout = String::from_utf8(output.stdout)?;
        let output_lines: Vec<&str> = stdout.lines().collect();

        if output_lines != self.expected {
            panic!(
                "Output mismatch. \n\
                 Pattern: {}\n\n\
                 Input:\n\
                 {}\n\n\
                 Expected:\n\
                 {}\n\n\
                 Got:\n\
                 {}",
                self.pattern,
                self.input.join("\n"),
                self.expected.join("\n"),
                output_lines.join("\n")
            );
        }

        Ok(())
    }
}

#[test]
fn basic_examples() -> anyhow::Result<()> {
    Test {
        args: &[],
        pattern: "hello",
        input: &["hello, world", "hi", "hello my dude"],
        expected: &["hello, world", "hello my dude"],
    }
    .run()?;

    // An empty pattern should return all entries.
    Test {
        args: &[],
        pattern: "",
        input: &["hi", "hey", "sup"],
        expected: &["hi", "hey", "sup"],
    }
    .run()?;

    Ok(())
}

#[test]
fn case_sensitivity() -> anyhow::Result<()> {
    Test {
        args: &["--case", "insensitive"],
        pattern: "HEL",
        input: &["hello", "Hello", "HELLOO", "heb"],
        expected: &["hello", "Hello", "HELLOO"],
    }
    .run()?;

    Test {
        args: &["--case", "sensitive"],
        pattern: "hel",
        input: &["hello", "Hello", "HELLOO", "heb"],
        expected: &["hello"],
    }
    .run()?;

    Test {
        args: &["--case", "sensitive"],
        pattern: "HEL",
        input: &["hello", "Hello", "HELLOO", "heb"],
        expected: &["HELLOO"],
    }
    .run()?;

    // Default behavior is to enable case sensitivity if there are any uppercase
    // letters in the pattern.
    Test {
        args: &[],
        pattern: "hel",
        input: &["hello", "Hello", "HELLOO", "heb"],
        expected: &["hello", "Hello", "HELLOO"],
    }
    .run()?;

    Test {
        args: &[],
        pattern: "HEL",
        input: &["hello", "Hello", "HELLOO", "heb"],
        expected: &["HELLOO"],
    }
    .run()?;

    Ok(())
}

// https://github.com/LPGhatguy/computergeneration/issues/1
#[test]
fn issue_1() -> anyhow::Result<()> {
    Test {
        args: &["--case", "insensitive"],
        pattern: "rojo.",
        input: &["rojo", "rojo.space"],
        expected: &["rojo.space"],
    }
    .run()?;

    Ok(())
}
