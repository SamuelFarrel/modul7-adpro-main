# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Tergantung pada bagaimana kita akan mengimplementasikan `Subscriber`. Jika kita ingin mengimplementasikan metode atau behavior yang berbeda-beda (unique) untuk jenis-jenis Subscriber, diperlukan adanya interface untuk Subscriber. Jika semua Subscriber akan memiliki behavior dan method yang sama maka a single Model struct sudah cukup.

2. Jika sebuah attribute harus unik, menggunakan `Vec` (list) mungkin tidak cukup karena `Vec` tidak secara otomatis memastikan keunikan elemennya. Demgam menggunakan `DashMap` (peta/dictionary) seperti yang sudah diimplementasikan, lebih tepat karena `DashMap` memastikan keunikan kunci dan mencegah duplikasi URL. Dengan menggunakan `DashMap`, terjamin bahwa setiap elemen hanya ada satu pada dalam struktur data karena ketika kita memasukkan elemen key yang sudah ada, maka valuenya akan diperbarui dengan yang baru.

3. Mengimplementasikan Singleton pattern secara langsung juga tetap dapat menjamin keamanan thread bagi program asalkan kita dapat mengimplementasikan secara mandiri dengan benar (dapat menggunakan `Mutex`), tidak seperti menggunakan DashMap yang merupakan library siap pakai yang sudah terjamin thread safetynya tanpa kita implementasikan sendiri.


#### Reflection Publisher-2
1. Pemisahan `Model` dengan `Service` dan `Repository` diperlukan agar tidak semua responsibility dihandle oleh `Model` saja, tetapi dibagi juga ke `Service` dan `Repository` sesuai dengan fungsinya masing-masing. Dengan begitu, pemisahan ini membuat aplikasi memenuhi kaidah clean code dan _Single Responsibility Principle_ sehingga kode akan lebih mudah dalam pemeliharaan dan pembaharuan.

2. Jika aplikasi hanya menggunakan `Model` saja, seluruh responsibility akan dihandle oleh model sehingga kode dari tiap model akan lebih kompleks dan panjang karena kode akan mengatur request dan response, struct data container, berisi method untuk business logic, dan menjadi database beserta method untuk mengaksesnya. Secara singkat, semua tugas masing-masing komponen MVC akan dihandle oleh kode dari `Model`.

3. Saya telah menggunakan Postman sejak semester lalu dan sudah lumayan mengeksplor fitur-fitur yang ada. Aplikasi ini biasanya saya gunakan untuk melakukan testing terhadap fungsionalitas aplikasi dengan mengirimkan request dan memeriksa response yang diberikan kembali oleh aplikasi. Fitur yang menurut saya akan berguna adalah `import documentation` yang juga kita gunakan untuk tutorial ini karena kita dapat menggunakan use case yang telah dibuat sebelumnya dan fitur fitur API testingnya secara keseluruhan.

#### Reflection Publisher-3
1. Pada tutorial, observer pattern yang digunakan adalah Push model karena ketika ada perubahan yang terjadi pada produk, `NotificationService` akan memberikan notifikasi ke subscriber tentang perubahan yang terjadi.

2. Keuntungan menggunakan pull model adalah subscriber lebih bebas untuk menentukan kapan mereka akan menerima notifikasi dan akan lebih efisien karena notifikasi akan dikirimkan jika diperlukan saja. Kekurangannya, diperlukan logic tambahan untuk menghandle pengambilan data notifikasi oleh subscriber dan adanya penundaan sampainya informasi kepada subscriber.

3. Jika kita tidak menggunakan multi-threading untuk notifikasi, masalah akan muncul ketika ada banyak subscriber yang perlu dinotify secara bersamaan karena proses notifikasi akan dilakukan secara berurutan, yang dapat menyebabkan penundaan dalam memberikan notifikasi kepada subscriber.
