<a href="https://andrewtbiehl.com">
  <img src="assets/img/andrew-profile.png" alt="Cartoon rendering of me (Andrew)."
    height="114px" align="right"/>
</a>

# [andrewtbiehl.com](https://andrewtbiehl.com)

**The personal website of [Andrew T. Biehl](https://andrewtbiehl.com/about)**

[![Deployment status](https://img.shields.io/github/workflow/status/andrewtbiehl/andrewtbiehl.com/Site%20Deployment?style=flat-square&label=deployment&logo=github)](https://github.com/andrewtbiehl/andrewtbiehl.com/deployments)
[![Website status](https://img.shields.io/website?style=flat-square&url=https://andrewtbiehl.com)](https://andrewtbiehl.com)

This is the source code for the static website
[andrewtbiehl.com](https://andrewtbiehl.com). It is a standard
[Jekyll](https://jekyllrb.com) site deployed to [GitHub Pages](https://pages.github.com)
via
[a custom GitHub Actions workflow](https://github.com/andrewtbiehl/andrewtbiehl.com/blob/main/.github/workflows/site-deployment.yaml).

## Contributing

Contributions are certainly welcome, albeit not particularly expected given the personal
nature of this project.

### Development

For a functional local development environment, make sure you have
[all the prerequisite software for Jekyll](https://jekyllrb.com/docs/installation),
[Bundler](https://bundler.io/), and [Rust](https://www.rust-lang.org/learn/get-started)
installed.

This project also relies on
[Git submodules](https://git-scm.com/docs/gitsubmodules#_description) for some of its
functionality. Accordingly, **make sure to initialize recursive submodules** when
cloning the project, for example with the following command:

```console
git clone --recurse-submodules https://github.com/andrewtbiehl/andrewtbiehl.com.git
```

In addition, one submodule does not function properly unless its dependencies are
resolved. The recommended workaround to address this is to add a symbolic link in this
submodule by running the following command in the root of the project after cloning:

```console
ln -s .. _tree_sitter_ruby_binding/parsers/tree-sitter-cpp/node_modules
```

Next, install the project's Ruby dependencies by running the following command from the
root of the project:

```console
bundle install
```

Next, build the Tree-sitter Ruby binding library by running the following command from
the `_tree_sitter_ruby_binding` directory:

```console
cargo build --release
```

Once the environment is set up, you can build, serve, and subsequently view the site
locally at `http://localhost:4000`, via the following command:

```console
bundle exec jekyll serve
```

Note that any time the Tree-sitter Ruby binding library is modified, it will need to be
rebuilt in the same way as described earlier in order for such changes will be reflected
in the site.

## Contact

Feel free to reach out if you have any comments or questions! My contact information can
be found [on the website](https://andrewtbiehl.com/about#contact-me).

## License

The content of this blog is licensed under the
[Creative Commons Attribution 4.0 International License](http://creativecommons.org/licenses/by/4.0).

The source code for this blog is licensed under the MIT license.

The text of both licenses can be found in
[this project's LICENSE.txt file](LICENSE.txt).
