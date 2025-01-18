# t
A very minimal, CLI based todo application

### Why?

- I wanted an easy way to manage my todo list using the CLI.
- I wanted to use Rust.

### How it works?

```markdown
Usage: t <COMMAND>

Commands:
  a     Adds the todo item
  d     Marks the todo item as done
  help  Print this message or the help of the given subcommand(s)
  l     Marks the todo item as done
  r     Removes the todo item
```

### 1. Add an item
`t a "update dependencies"`

```markdown
Usage: t a <ITEM>

Arguments:
  <ITEM>  Id of the todo item
```

2. List items

`t l`

3. Mark an item as done

`t d 1` (this marks the first item in the todo as done)

```markdown
Usage: t d <ID>

Arguments:
  <ID>  Id of the todo item
```

4. Remove an item

`t r t 1` (this removes the first item from the todo bucket)

`t r d 1` (this removes the first item from the done bucket)

```markdown
Usage: t r <BUCKET> <ID>

Arguments:
  <BUCKET>  Bucket to remove from (t for todo, d for done)
  <ID>      Id of the todo item
```
