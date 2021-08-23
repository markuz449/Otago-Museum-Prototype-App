/**
 * Async Functions, handles connections to the API
 */

// Gets a question from the API:
async function getQuestion(url = '') {
    // Default options are marked with *
    const response = await fetch(url, {
        method: 'GET',
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
    });
    return response.json();
}

// Checks to see if they clicked the correct answer:
async function checkAnswer(url = '', data = {}) {
    // Default options are marked with *
    const response = await fetch(url, {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        credentials: 'same-origin',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });
    return response.json();
}


/**
 * Listener functions for the buttons
 */
function answer(answer_button) {
    if (answered_flag){
        return;
    } 
    let url = "http://188.166.185.37:8080/api/check-answer/" + question_object.question_id;
    checkAnswer(url, answer_button.innerHTML).then((value) => {
        if (value == "correct"){
            document.getElementById(answer_button.id).style.backgroundColor = "lime";
            score += 1;
            if (score >= 8){
                location.href="congrats.html";
            } else {
                document.getElementById("score").innerHTML = score + "/8"
            }
        } else {
            document.getElementById(answer_button.id).style.backgroundColor = "red";
            document.getElementById("answer1").style.backgroundColor = "lime";
        }
        document.getElementById("next").style.display = "block";
    });
    answered_flag = true;
}

function next(){
    if (!answered_flag){
        return;
    }
    setQuestion();
    answered_flag = false;
    document.getElementById("next").style.display = "none";
}

function setQuestion(){
    getQuestion("http://188.166.185.37:8080/api/question").then((question) => {
        document.getElementById("question").innerHTML = question.question_text;
        document.getElementById("answer1").innerHTML = question.correct_answer;
        document.getElementById("answer2").innerHTML = question.wrong_answer_1;
        document.getElementById("answer3").innerHTML = question.wrong_answer_2;
        document.getElementById("answer4").innerHTML = question.wrong_answer_3;
        document.getElementById("answer1").style.backgroundColor = "aliceblue";
        document.getElementById("answer2").style.backgroundColor = "aliceblue";
        document.getElementById("answer3").style.backgroundColor = "aliceblue";
        document.getElementById("answer4").style.backgroundColor = "aliceblue";
        question_object = question;
    });
}

/**
 * On load script code - runs when the page is loaded
 */
console.log("Getting question data from the api...");
var answered_flag = false;
var question_object;
var score = 0;
var age = sessionStorage.getItem("age");
if (age == null){
    age = 0;
}
console.log("Age: " + age);
setQuestion();