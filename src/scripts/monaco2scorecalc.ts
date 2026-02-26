
const coins_per_level: Map<string, number> = new Map([
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

const calc_form = document.getElementById("score_calculator_form") as HTMLFormElement;
const score_output = document.getElementById("score_output") as HTMLInputElement;

function getCoinsPerlevel(): number
{
    let selected_level = document.getElementById("level_name") as HTMLInputElement;
    return coins_per_level.get(selected_level.value) || 0;
}

function onLevelChange(): void
{
    //figure out what level was just changed to
    let selected_level: string = (document.getElementById("level_name") as HTMLInputElement).value;

    //update the max coins in the coin selector
    let coin_selector = document.getElementById("coins_collected") as HTMLInputElement;
    coin_selector.max = String(coins_per_level.get(selected_level));

    let max_coins_display = document.getElementById("max_coins_display") as HTMLInputElement;
    max_coins_display.innerText = "/" + String(coins_per_level.get(selected_level));

}

function preventNonNumericalInput(event: KeyboardEvent): void
{
    let reg = new RegExp("[0-9]+");
    if(!event.key.match(reg))
    {
        event.preventDefault();
    }
}

function checkOverflow(input: string): void
{
    let input_element = document.getElementById(input) as HTMLInputElement;
    if(parseInt(input_element.value) > parseInt(input_element.max))
    {
        input_element.value = input_element.max;
    }
}

function checkValidInput(): void
{

}

function getPlayerMulti(): number
{
    for(let i=1; i<=4; i++)
    {
        let element = document.getElementById(i+"_player") as HTMLInputElement;
        if(element.checked == true)
        {
            return parseFloat(element.value);
        }
    }

    return 0;
}

function calculateScore(): number
{
    let hours_time = parseInt((document.getElementById("clear_time_hours") as HTMLInputElement).value);
    let minutes_time = parseInt((document.getElementById("clear_time_minutes") as HTMLInputElement).value);
    let seconds_time = parseInt((document.getElementById("clear_time_seconds") as HTMLInputElement).value);
    let milliseconds_time = parseInt((document.getElementById("clear_time_milliseconds") as HTMLInputElement).value);

    let total_time = hours_time * 3600 + minutes_time * 60 + seconds_time + milliseconds_time/100;

    let time_under_hour = 3600 - total_time;
    if(time_under_hour < 0)
    {
        time_under_hour = 0;
    }

    let coins_collected = parseInt((document.getElementById("coins_collected") as HTMLInputElement).value);

    let result = time_under_hour + coins_collected * (getPlayerMulti() * 4);

    console.log(result);

    return result * 100;

}

function updateScore(): void
{
    score_output.value = calculateScore().toString();
}

calc_form.addEventListener("input", updateScore);
updateScore();
