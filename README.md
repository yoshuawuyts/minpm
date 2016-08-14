# minpm
npm, but for tiny machines

## Usage
```txt
minpm(1) - install dependencies from npm

Usage: minpm install <package.json>

Flags:
  -h, --help  Print usage

Commands:
  install   install dependencies

Examples:
  minpm install   # install dependencies
```

## Rant
`npm install` doesn't work reliably on machines that have less than a couple of
gig worth of RAM floating around. I think it's silly that that's a requirement
for something that's in essence a glorified `curl(1)` command. Sure, optimizing
is important but before optimizing it's important for stuff to work at all. And
it doesn't.

## License
[MIT](https://tldrlegal.com/license/mit-license)
