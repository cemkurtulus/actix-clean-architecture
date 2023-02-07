<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://i.imgur.com/6wj0hh6.jpg" alt="Project logo"></a>
</p>

<h3 align="center">The hexagonal architecture for rust</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/kylelobo/The-Documentation-Compendium.svg)](https://github.com/cemkurtulus/actix-clean-architecture/issues)

</div>

---

<p align="center"> This project is an example of hexagonal architecture in rust.
    <br> 
</p>

## 📝 Table of Contents

- [Technologies](#technologies)
- [Dependency](#dependency)
- [Getting Started](#getting_started)
- [Authors](#authors)


## 🚀 Technologies <a name = "dependency"></a>

- [MongoDB](https://www.mongodb.com/)
- [Actix Web](https://github.com/actix/actix-web)


## 🎈 Dependency <a name = "dependency"></a>

- actix-web = "4.3.0"
- log = "0.4.0"
- env_logger = "0.9.0"
- serde = { version = "1.0", features = ["derive"] }
- mongodb = "2.3.0"
- dotenv = "0.15.0"
- async-trait = "0.1.63"

## 🏁 Getting Started <a name = "getting_started"></a>

Should add ```.env``` file add
```
MONGOURI:
```

Run command
```
cargo run
```

Add User Request
```
curl --location --request POST 'http://localhost:8080/user' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name": "Cem",
    "surname": "Kurtulus"
}'
```

Get User Request
```
curl --location --request GET 'http://localhost:8080/user/63e00caf27cd9de8246778a0'
```

## ✍️ Authors <a name = "authors"></a>

- [@cemkurtulus](https://github.com/cemkurtulus) - Idea & Initial work

