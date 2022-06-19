# Running

## Front end
Install [yarn](https://classic.yarnpkg.com/en/docs/install/#mac-stable)

To run the frontend:
```bash
cd web
yarn dev
```



## Back end
[Install Rust/Cargo](https://www.rust-lang.org/tools/install)

Then install `cargo-watch` with
```bash
cargo install cargo-watch
```

To run the backend:
```bash
cd api
cargo watch -x run
```

# Tasks
Address `TODO` comments.

Begin with those related to `GeneViewer` in `web`.
The gene viewer should show genes as blocks on a line.

Something along the lines of (just a single one of the lines with blocks, buttons not necessary at all):

![vis](https://i.ytimg.com/vi/saIIaETgUW0/maxresdefault.jpg)

The molecular viewer is already given `x` and `y` coordinates of the atoms in a molecular structure.
Bonds are represented as pairs of atom indices and a multiplicity denoting the order of the bond (single, double, triple, etc.).
An example molecule drawing is here:

![mol](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2F36gu5d4dxary1824ba1o7kkq6uc-wpengine.netdna-ssl.com%2Fhelp%2Fwp-content%2Fuploads%2Fsites%2F2%2F2019%2F06%2FCompound1.png&f=1&nofb=1)

Note that carbons are not explicitly shown but all other atoms are.

If you have questions please email me at `mguler@cmu.edu`.

Follow best git and software development practices.
