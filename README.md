# plier [![Build Status](https://travis-ci.org/shaolang/plier.svg?branch=master)](https://travis-ci.org/shaolang/plier)

> ply: verb (used with object), plied, ply-ing
>
> to work with or at diligently; employ busily; use
>
> plier: noun
>
> a person or thing that plies

plier is a command line tool that eases the switching of development tools
between versions by manipulating the path and environment variables.
The goal is to make this a generic enough tool to replace the plethora
of version managers for different languages.

Non-goal: alias commands to do automatic switching,

## Usage

To add application for plier to manage:

```
$ plier add --app java --home-name JAVA_HOME --bin bin
```

Then to register an installation of the application:

```
$ plier add --app java --version 11 --home-path path/to/java/home
```

To use the application, specify the application followed by the version after
the `use` subcommand:

```
$ plier use java 11
```

To see all the registered applications:

```
$ plier list
```

Copyright © 2020 Shaolang Ai

Distributed under the Apache Software License 2.0
