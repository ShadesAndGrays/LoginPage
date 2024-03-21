let username = ""
let password = ""

function getInfo(){
    username = document.getElementById("username").value;
    password = document.getElementById("password").value;
}

loginBtn = document.getElementById("login")
loginBtn.addEventListener("click",() => {
    login()
})


async function login(){
    getInfo()

    if (username === ""){
        alert("Username is empty")
        return
    }
    if(password === ""){
        alert("Password is empty")
        return
    }
    try{
    let value = await fetch(`http://localhost:8000/user/${username}`);
    let jsonValue = await value.json();
    if (jsonValue.length === 0){
        return
    }

    if (password === jsonValue.password){
        alert("Login was successful")
    }
    else{
        alert("Failed to Login")
    }}catch(error){
        console.error("Error: ",error)
        alert("An error occured during login")

    }
}
async function signup(){
    getInfo()
    // Copy pasted code I would be lying if I said Iknew 
    try{
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
    }catch(error) {
        console.error("Error: ",error)
        alert("An error occured during sign up")

    }
}
