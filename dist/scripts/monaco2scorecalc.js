"use strict";
const coins_per_level = new Map([
    ["yacht_club", 130],
    ["hotel", 169],
    ["bona_fide_sanitation", 270],
    ["cocktail_party", 317],
    ["prison", 231],
    ["museum", 296],
    ["shipyard", 362],
    ["data_center", 300],
    ["night_club", 399],
    ["opera", 294],
    ["bank", 403],
    ["palace", 337],
    ["safehouse", 310],
    ["catacombs", 525],
    ["casino", 464],
    ["bonhomme", Number.MAX_SAFE_INTEGER],
    ["petit_bank_optional", 315],
    ["museum_optional", 271],
    ["office_optional", 268], //unconfirmed
    ["art_gallery_optional", 274], //unconfirmed
]);
const calc_form = document.getElementById("score_calculator_form");
const score_output = document.getElementById("score_output");
function getCoinsPerlevel() {
    let selected_level = document.getElementById("level_name");
    return coins_per_level.get(selected_level.value) || 0;
}
function onLevelChange() {
    //figure out what level was just changed to
    let selected_level = document.getElementById("level_name").value;
    //update the max coins in the coin selector
    let coin_selector = document.getElementById("coins_collected");
    coin_selector.max = String(coins_per_level.get(selected_level));
    let max_coins_display = document.getElementById("max_coins_display");
    max_coins_display.innerText = "/" + String(coins_per_level.get(selected_level));
    //ensure that the coins collected is not too high (can occur when switching from a level with a higher max coins to a level with a lower max coins)
    checkOverflow('coins_collected');
}
function preventNonNumericalInput(event) {
    let reg = new RegExp("[0-9]+");
    if (!event.key.match(reg)) {
        event.preventDefault();
    }
}
function checkOverflow(input) {
    let input_element = document.getElementById(input);
    if (parseInt(input_element.value) > parseInt(input_element.max)) {
        input_element.value = input_element.max;
    }
}
function checkValidInput() {
}
function getPlayerMulti() {
    for (let i = 1; i <= 4; i++) {
        let element = document.getElementById(i + "_player");
        if (element.checked == true) {
            return parseFloat(element.value);
        }
    }
    return 0;
}
function calculateScore() {
    let hours_time = parseInt(document.getElementById("clear_time_hours").value);
    let minutes_time = parseInt(document.getElementById("clear_time_minutes").value);
    let seconds_time = parseInt(document.getElementById("clear_time_seconds").value);
    let milliseconds_time = parseInt(document.getElementById("clear_time_milliseconds").value);
    let total_time = hours_time * 3600 + minutes_time * 60 + seconds_time + milliseconds_time / 100;
    let time_under_hour = 3600 - total_time;
    if (time_under_hour < 0) {
        time_under_hour = 0;
    }
    let coins_collected = parseInt(document.getElementById("coins_collected").value);
    let result = time_under_hour + coins_collected * (getPlayerMulti() * 4);
    console.log(result);
    return result * 100;
}
function updateScore() {
    score_output.value = calculateScore().toString();
}
calc_form.addEventListener("input", updateScore);
updateScore();
