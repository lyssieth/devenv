# devenv

It's a shitty devenv helper program.

`/files` contains some example files to poke at.

## Purpose

The purpose of this project is to help with the initial setup of a new project. Like creating templated CI/CD pipelines for testing/linting/whatever, base `Dockerfile`s, `Justfile`s or `Makefile`s, etc.

## Usage

The root of all commands is

```bash
devenv
```

On first run, it will create a default configuration file in its directory.

The default language is `any` (`-l <language>`) and the platform is `any` (`-p <platform>`).  
Both of which need to exist in the configuration file under their respective headings in order to function, however those two exist as defaults in the generated config file.

`any` is a hard-coded wildcard and will is always a valid language/platform. However, one does need to have a template file for `any` added via `devenv create`.

Note: All arguments are case-sensitive (for now)

### `devenv create`

This creates new template files for usage with `devenv generate`.

As an example, creating a template Dockerfile.

```bash
devenv create docker ./Dockerfile
# This creates a new template Dockerfile for `any` language projects for the `any` platform.
# However, a more likely use case is:
devenv -l rust create docker ./Dockerfile
# Which creates a template Dockerfile for `rust` projects on `any` platform.
```

Templates can currently contain the following placeholders:

- `{ProjectName}`: The name of the project (essentially the current working directory, however in the future maybe the name of the git repo or topmost directory that's a git repo)
- `{ProjectName_DashesToUnderscores}`: The name of the project with dashes replaced with underscores
- `{ProjectName_Lowercase}`: The name of the project with all characters lowercase

### `devenv generate`

This generates a file based on an existing template.

As an example, generating a Dockerfile for a `rust` language project.

```bash
devenv -l rust generate docker
```

If the template is:

```Dockerfile
FROM rust:latest

WORKDIR /{ProjectName}

COPY . .

CMD [ "cargo", "run", "--release" ]
```

Then the output file, assuming the project name is `rust-project`:

```Dockerfile
FROM rust:latest

WORKDIR /rust-project

COPY . .

CMD [ "cargo", "run", "--release" ]
```

Written to a file called `Dockerfile`

### `devenv config`

This is mostly for debug reasons, however it can be helpful.

#### `devenv config path`

This will print the path to the configuration file.

```bash
$ devenv config path
/home/user/.config/devenv/config.yml
```

#### `devenv config regenerate`

This will regenerate the configuration file, using the default defined in [config.rs](src/config.rs)

#### `devenv config show`

This prints out the current config file as a pretty-printed Rust struct.

## Problems

- It's probably got bugs. I haven't tested it much. It works in my use-case, and I haven't had reason to test it.
- It's hosted here on <https://git.lys.ee>, where nobody else can add issues or w/e.
