# ja (just ask) a small CLI app that allows you to work with AI tools

Right now it's just yet another wrapper around the openAI apis

Run interactively:

```shell
ja
```

![demo](https://github.com/joshka/ja/assets/381361/c79a3788-17fa-4ca1-97c8-5f4f3d5ed09f)

Command line parameters

```text
ja (just ask) is a small CLI / TUI app that allows you to work with AI tools

Usage: ja [OPTIONS] [-- <MESSAGE>...] [COMMAND]

Commands:
  chat  default command (can be omitted)
  logs  
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [MESSAGE]...
          

Options:
  -m, --model <MODEL>
          ID of the model to use

          Possible values:
          - gpt-3.5-turbo:          alias 3.5 (default - because it's fast and cheap)
          - gpt-3.5-turbo-0301
          - gpt-3.5-turbo-0613
          - gpt-3.5-turbo-1106
          - gpt-3.5-turbo-16k:      alias 3.5-16k
          - gpt-3.5-turbo-16k-0613
          - gpt-4:                  alias 4
          - gpt-4-0314
          - gpt-4-0613
          - gpt-4-1106-preview
          - gpt-4-32k:              alias 4-32k
          - gpt-4-32k-0314
          - gpt-4-32k-0613

  -t, --temperature <TEMPERATURE>
          Sampling temperature

  -p, --top-p <TOP_P>
          Probability mass percentage

  -n, --n <N>
          Number of choices to generate for each input message

  -s, --stream <STREAM>
          Stream the resultt of the API call
          
          [possible values: true, false]

      --stop <STOP>
          Up to 4 sequences where the API will stop generating further tokens

  -c, --max-tokens <MAX_TOKENS>
          The maximum number of tokens to generate in the chat completion

      --presence-penalty <PRESENCE_PENALTY>
          Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far

      --frequency-penalty <FREQUENCY_PENALTY>
          Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far

  -u, --user <USER>
          A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse

  -i, --input <INPUT>
          Input file

  -o, --output <OUTPUT>
          Output file

      --system <SYSTEM>
          System message

  -v, --verbose
          Show verbose output

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Example usage:

Provide command line paramters directly:

```text
$ ja -- hello
assistant: Hello! How can I assist you today?
```

Provide input via a pipe:

```text
$ echo "hello" | ja
assistant: Hello! How can I assist you today?
```

## TODO

In no specific order, some ideas:

- [x] Store logs in cache dir (use --verbose to see the filename)
- [x] Accept input from stdin
- [x] Prompts for input at the terminal if none is provided from command line / stdin
- [x] Chat mode. Not just a single conversation, but multiple (there are plenty of ChatGPT style apps already though)
- [ ] Syntax highlight code in output
- [ ] Add a TUI mode (using [ratatui](https://tui-rs-revival/ratatui))
- [ ] History management: Allow users to view, search, and manage their conversation history, making it easier to refer back to previous interactions.
- [ ] Customizable output formatting: Provide options for users to customize the output format, such as plain text, JSON, or even HTML, to better suit their needs.
- [ ] Rate limiting and usage monitoring: Add features to monitor API usage and enforce rate limits, helping users avoid unexpected costs or API restrictions.
- [ ] Support for other OpenAI models: Expand the app to support additional OpenAI models, giving users more options for generating content.
- [ ] Text-to-speech integration: Integrate text-to-speech functionality, allowing users to listen to the AI-generated responses instead of reading them.
- [ ] Customize system message. Customizable AI behavior: Allow users to adjust the AI's behavior
- [ ] Templated messages
