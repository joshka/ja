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

In no specific order, some ideas:

- [x] Store logs in cache dir (use --verbose to see the filename)
- [x] Accept input from stdin
- [x] Prompts for input at the terminal if none is provided from command line / stdin
- [ ] Syntax highlight code in output
- [ ] Add a TUI mode (using [ratatui](https://tui-rs-revival/ratatui))
- [ ] Chat mode. Not just a single conversation, but multiple (there are plenty of ChatGPT style apps already though)
- [ ] History management: Allow users to view, search, and manage their conversation history, making it easier to refer back to previous interactions.
- [ ] Customizable output formatting: Provide options for users to customize the output format, such as plain text, JSON, or even HTML, to better suit their needs.
- [ ] Rate limiting and usage monitoring: Add features to monitor API usage and enforce rate limits, helping users avoid unexpected costs or API restrictions.
- [ ] Support for other OpenAI models: Expand the app to support additional OpenAI models, giving users more options for generating content.
- [ ] Text-to-speech integration: Integrate text-to-speech functionality, allowing users to listen to the AI-generated responses instead of reading them.
- [ ] Customize system message. Customizable AI behavior: Allow users to adjust the AI's behavior
- [ ] Templated messages
