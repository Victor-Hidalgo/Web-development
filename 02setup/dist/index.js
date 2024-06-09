"use strict";
// class User {
//     public email: string
//     private name: string
//     readonly city: string = "New York"
//     constructor(email: string, name: string) {
//         this.email = email;
//         this.name = name;
//     }
// }
// const victor = new User("hidalgo@dev.com", "Victor");
class User {
    constructor(email, name) {
        this.email = email;
        this.name = name;
        this.city = "New York";
    }
}
const victor = new User("hidalgo@dev.com", "Victor");
