let cargo = toml(readFile("./Cargo.toml"))
let package = cargo.package

let name = package.name

package({
    id = name
    version = package.version
    description = maybe(package.description "No description")
    authors = maybe(package.authors [ ])

    src = "."
    expected_output = [
        "/bin/${name}"
    ]

    build = "cargo build --release" 
    install = "cp target/release/${name} /bin/${name}"
})
