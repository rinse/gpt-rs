# GPT-rs

Embed GPTs in Unix philosophy.

## What is it

GPT-rs supplies an instant heuristic CLI tool.
Your new tool is characterized by a prompt, takes inputs from stdin or arguments and prints a result on the stdout.

## Getting a new tool

You have a new tool in `.bash_aliases` or somewhere.

Below shows an example that you get a `newtool`.

```bash
alias newtool='gpt-rs --prompt "$(cat $HOME/.config/gpt-rs/prompt-newtool.txt)"'
```

Using your `newtool`, it is required to have the `CHATGPT_API_KEY` environment variable.

```bash
export CHATGPT_API_KEY=***
```

Then you can use your new tool.

Takes your inputs from stdin:

```bash
$ cat inputs.txt | newtool -
```

Or takes your inputs from arguments:

```bash
newtool $(cat inputs.txt)
```
