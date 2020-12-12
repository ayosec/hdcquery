# hdcquery

CLI tool to query data from [hub.docker.com](https://hub.docker.com).

## Demo

<div style="text-align: center">

[![Demo for hdcquery](https://asciinema.org/a/GID4GzyH6HYYzvmlEkqF6eiTK.svg)](https://asciinema.org/a/GID4GzyH6HYYzvmlEkqF6eiTK)

</div>

## Changelog

Please see the [CHANGELOG](./CHANGELOG.md).

## Usage

### Search for repositories

With `hdcquery search` you can search for repositories:

```console
$ hdcquery search redis
     IMAGE                          DESCRIPTION                                               LAST UPDATE  PULLS  STARS
   1 redis                          Redis is an open source key-value store that …            8 hours ago   10M+   8854
   2 rediscommander/redis-commander Alpine image for redis-commander - Redis …                 9 days ago   10M+     48
   3 redislabs/redisearch           Redis With the RedisSearch module pre-loaded. See …        3 days ago    1M+     29
   4 redislabs/redisinsight         RedisInsight - The GUI for Redis                          16 days ago    1M+     15
   5 redislabs/redismod             An automated build of redismod - latest Redis …            3 days ago  500K+      7
   6 redislabs/rejson               RedisJSON - Enhanced JSON data type processing …          30 days ago  500K+     22
   […]
```

After every page you can type a number to show more details of one of the results, or go to the next page of the search.

    [Found 14432 results] <ENTER>: more results | Number and <ENTER>: image details >

You can limit the numbers of results with the `-l` / `--limit` option.

<details>
<summary>Options for <code>search</code>.</summary>

```console
$ hdcquery --help search
Usage: hdcquery search [OPTIONS]

Positional arguments:
  terms

Optional arguments:
  -l, --limit LIMIT  Limit the number of results
  -s, --search-url SEARCH-URL
                     URL to send search requests
```
</details>


### Show repository details

`hdcquery show` can show the metadata and the description of a repository:

```console
$ hdcquery show redis
Namespace: library
Name: redis
Description: Redis is an open source key-value store that functions as a data structure server.
Starts: 8853
Pulls: 2147483647
Automated: no
Last updated: 2020-12-11 22:02 UTC (10 hours ago)

----

# Quick reference

-	**Maintained by**:
	[the Docker Community](https://github.com/docker-library/redis)
[…]
```

You can skip the metadata and show only the description of the repository with the `-o` / `--only-description` option.

By default, the output of the `show` command is sent to a pager (like `less(1)`). You can use your own pager setting the `HDC_PAGER` variable.

For example, to enable syntax highlighting with [Pygments](https://pygments.org/):

```
$ export HDC_PAGER='sh -c "pygmentize -l md | less -FR"'
```

The pager receives two environment variables:

* `HDCQUERY_REPOSITORY`: the name of the repository (like `library/redis`).
* `HDCQUERY_VERSION`: the version of hdcquery.

<details>
<summary>Options for <code>show</code>.</summary>

```
$ hdcquery --help show
Usage: hdcquery show [OPTIONS]

Positional arguments:
  repositories

Optional arguments:
  -o, --only-description  Only show full description
```
</details>

### List image tags

`hdcquery tags` list tags available in a repository.

```console
$ hdcquery tags redis
- 421 results for redis
SIZE       OS       ARCH   LAST PUSHED     NAME
36.7 MiB   linux    386    10 hours ago    latest
31.5 MiB   linux    arm    10 hours ago    latest
33.8 MiB   linux    arm    10 hours ago    latest
35.1 MiB   linux    arm64  10 hours ago    latest
36.4 MiB   linux    amd64  10 hours ago    latest
35.1 MiB   linux    s390x  10 hours ago    latest
35.1 MiB   linux    arm64  10 hours ago    buster
[…]
```

The option `-l` / `--limit` limits the number of results to show.

The option `-a` / `--architecture` filters the results to the given architecture (`amd64`, `arm`, `386`, …).

The option `-o` / `--operating-system` filters the results to the given operating system (`linux` or `windows`).

The option `-c` / `--current-machine` filters the results to match the operating system and the architecture of the machine where the tool is executed.

Finally, the option `-d` / `--digest` shows the image digest for every tag:

```console
$ hdcquery tags -l4 -dc redis
- 421 results for redis
SIZE       OS       ARCH   LAST PUSHED    DIGEST                                                                    NAME
36.4 MiB   linux    amd64  10 hours ago   sha256:466da50d1e0ba009816a4b507a9b526a34169e026e967f304679b1553cbca66c   latest
36.4 MiB   linux    amd64  10 hours ago   sha256:466da50d1e0ba009816a4b507a9b526a34169e026e967f304679b1553cbca66c   buster
10.1 MiB   linux    amd64  10 hours ago   sha256:4920debee18fad71841ce101a7867743ff8fe7d47e6191b750c3edcfffc1cb18   alpine3.12
10.1 MiB   linux    amd64  10 hours ago   sha256:4920debee18fad71841ce101a7867743ff8fe7d47e6191b750c3edcfffc1cb18   alpine
```

<details>
<summary>Options for <code>tags</code>.</summary>

```console
$ hdcquery --help tags
Usage: hdcquery tags [OPTIONS]

Positional arguments:
  repositories

Optional arguments:
  -l, --limit LIMIT      Limit the number of results (default: 30)
  -d, --digest           Show image digest
  -a, --architecture ARCHITECTURE
                         Filter by architecture
  -o, --operating-system OPERATING-SYSTEM
                         Filter by operating system
  -c, --current-machine  Filter by operating system and architecture of this machine
```
</details>

## Installation

### Pre-built binaries

Every release includes binaries for Linux, macOS, and Windows.

You can download them in the [*Releases* page](https://github.com/ayosec/hdcquery/releases).

The binaries are built in a [GitHub Actions workflow](./.github/workflows/release.yml), so they are limited to the platforms supported by GitHub.

### Using Docker

You can run the tool with Docker, using the `ayosec/hdcquery` image.

```console
$ docker run --rm ayosec/hdcquery --help
```

Like the pre-built binaries, the Docker image is built in another [GitHub Actions workflow](./.github/workflows/docker.yml) for every new release.

### From sources

To build the tool from sources:

1. [Install the Rust toolchain](https://www.rust-lang.org/tools/install).

2. Use `cargo install`:

    ```console
    $ cargo install --git https://github.com/ayosec/hdcquery.git
    ```

## Similar tools

* [amalfra/docker-hub](https://github.com/amalfra/docker-hub)
* [containers/skopeo](https://github.com/containers/skopeo)
