// Sets the age to session storage
function setAge(){
  sessionStorage.setItem("age", document.getElementById("input-age").value);
  location.href="question.html";
}