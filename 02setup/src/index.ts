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

    protected _courseCount = 1

    readonly city: string = "New York"

    constructor(
        public email: string, 
        public name: string,
        // private userId: string
    ) {
        
    }
    private deleteToken() {
        console.log("Token deleted");
        
    }

    get getAppleEmail(): string {
        return `apple${this.email}`
    }

    get courseCount(): number {
        return this._courseCount
    }

    set courseCount(courseNum) {
        if (courseNum <= 1) {
            throw new Error("Course count should be more than 1")
        }
        this._courseCount = courseNum
    }
}

class SubUser extends User {
    isFamily: boolean = true
    changeCourseCount() {
        this._courseCount = 4
    }
}

const victor = new User("hidalgo@dev.com", "Victor");
