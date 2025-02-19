# SeaORM Pro

<img src="https://www.sea-ql.org/sea-orm-pro/img/01_banner.png" width="100%"/>


## Installation

1. Modify `.env` with your database credentials, here it uses SQLite as the default database

```sh
# DATABASE_URL="mssql://sa:YourStrong()Passw0rd@localhost/AdventureWorksLT2016"
# DATABASE_URL="mysql://root:root@localhost/AdventureWorksLT2016"
# DATABASE_URL="postgres://root:root@localhost/adventureworkslt2016"
DATABASE_URL="sqlite://db.sqlite?mode=rwc"
```

2. If you want to target other databases, please enable the database backend in `Cargo.toml` accordingly:

```toml
[features]
default = ["sqlx-sqlite"] # <- change this
sqlx-mysql = ["sea-orm/sqlx-mysql"]
sqlx-postgres = ["sea-orm/sqlx-postgres"]
sqlx-sqlite = ["sea-orm/sqlx-sqlite"]
```

3. Setup database schema and seed the database

```sh
cargo run task seed_data

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/sea-orm-pro-backend-cli task seed_data`

Reading: "Customer.csv"
Seeding Completed: "Customer.csv"
Reading: "Address.csv"
Seeding Completed: "Address.csv"
Reading: "CustomerAddress.csv"
Seeding Completed: "CustomerAddress.csv"
Reading: "SalesOrderHeader.csv"
Seeding Completed: "SalesOrderHeader.csv"
Reading: "ProductCategory.csv"
Seeding Completed: "ProductCategory.csv"
Reading: "ProductModel.csv"
Seeding Completed: "ProductModel.csv"
Reading: "Product.csv"
Seeding Completed: "Product.csv"
Reading: "SalesOrderDetail.csv"
Seeding Completed: "SalesOrderDetail.csv"
Reading: "ProductDescription.csv"
Seeding Completed: "ProductDescription.csv"
Reading: "ProductModelProductDescription.csv"
Seeding Completed: "ProductModelProductDescription.csv"
All Tables Completed Seeding!
```

4. Download the artifact of admin panel frontend

```sh
sh build_tools/download_frontend.sh
```

5. Start the Loco.rs server

```sh
cargo run start

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/sea-orm-pro-backend-cli start`

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: logging, automigrate
     logger: debug
compilation: debug
      modes: server

listening on [::]:8086
```

6. Visit [http://localhost:8086/admin](http://localhost:8086/admin) and login with the credential

```
Email: demo@sea-ql.org
Password: demo@sea-ql.org
```

![](docs/demo_login.png)

![](docs/demo_table_list.png)

![](docs/demo_table_view.png)

## License

Licensed under MIT license ([LICENSE](https://github.com/SeaQL/sea-orm-pro/blob/main/LICENSE) or <http://opensource.org/licenses/MIT>)

