# ja (just ask) a small CLI app that allows you to work with AI tools

Right now it's just yet another wrapper around the openAI apis

```text
Usage: ja [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...
          

Options:
  -n, --max-tokens <MAX_TOKENS>
          [default: 1000]

  -m, --model <MODEL>
          [default: gpt-3.5-turbo]

          Possible values:
          - gpt-3.5-turbo: alias 3.5
          - gpt-4:         alias 4

  -t, --temperature <TEMPERATURE>
          [default: 0.7]

  -v, --verbose
          

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Example usage:

Provide command line paramters directly:

```text
$ ja hello
assistant: Hello! How can I assist you today?
```

Provide input via a pipe:

```text
$ echo "hello" | ja
assistant: Hello! How can I assist you today?
```

Run interactively:

```text
$ ja
Enter your input (finish with ^D):
hello
assistant: Hello! How can I assist you today?
```

## TODO

- [x] stores logs in cache dir (use --verbose to see the filename)
- [x] accept input from stdin
- [x] interactive mode
- [ ] syntax highlight code
- [ ] tui mode
