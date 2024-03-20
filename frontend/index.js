// You can tell I am not used to javascipt
let username = ""
let password = ""

function getInfo(){
    username = document.getElementById("username").value;
    password = document.getElementById("password").value;
}

async function login(){
    getInfo()

    if (username === ""){
        console.log("Username is empty")
        return
    }
    if(password === ""){
        console.log("Password is empty")
        return
    }
    let value = await fetch(`http://localhost:8000/user/${username}`);
    let jsonValue = await value.json();
    if (jsonValue.length === 0){
        console.log("Empty")
        return
    }
    else{
        console.log(jsonValue.length)
        console.log(username)
    }


    if (password === jsonValue.password){
        console.log("You have Login in")
    }
    else{
        console.log("Incorrect Password")
    }
}
async function signup(){
    getInfo()
    // Copy pasted code I would be lying if I said Iknew 
    fetch("http://localhost:8000/user", {
        method: "POST",
        body: JSON.stringify({
            name:username,
            password
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    })
        .then((response) => response.json())
        .then((json) => console.log(json));
}
