# Init the project

Clone this repo with fetching all the data of submodules

```shell
git clone --recurse-submodules https://github.com/Cykooz/libheif-sys.git
```

Or init and fetch all the data of submodules if you already have cloned the
repo without submodules:

```shell
git submodule init
git submodule update
```

# Pull changes with submodules updating

```shell
git pull --recurse-submodules
```

# Update a version of embedded libheif

- Fetch from a remote repository:
  ```shell
  git -C vendor/libheif fetch origin
  ```
- Checkout to required tag:
  ```shell
  git -C vendor/libheif/ checkout v1.19.8
  ```
