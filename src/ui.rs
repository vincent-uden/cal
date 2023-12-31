use maud::{html, Markup};

pub fn layout(child: Markup) -> Markup {
    html! {
        script src="https://cdn.tailwindcss.com" {}
        script src="/htmx.js" {}
        link rel="stylesheet" type="text/css" href="/root.css" {}
        body class="bg-zinc-700" {
            div class="mx-auto max-w-screen-lg text-zinc-100 px-8" {
                (child)
            }
        }
    }
}

pub fn sign_up_page(err: Option<&str>) -> Markup {
    html! {
        div id="sign-up-page" {
            div class="h-56" {}
            form
                id="signup-form"
                hx-post="/sign-up"
                hx-target="#sign-up-page"
                class="w-96 mx-auto"
            {
                div class="flex flex-col justify-center items-stretch" {
                    label class="w-full mb-2 font-bold"{ "Username" }
                    input id="username" class="bg-zinc-800 px-4 py-2 rounded-lg" name="username" {}
                    div class="h-8" {}
                    label class="w-full mb-2 font-bold" { "Password" }
                    input id="password" class="bg-zinc-800 px-4 py-2 rounded-lg" type="pasword" name="password" {}
                    div class="h-8" {}
                    input
                        type="submit"
                        value="Sign up"
                        class="px-4 py-2 bg-green-500 rounded-lg hover:bg-green-400"
                        {}
                    div class="h-8" {}
                    p class="text-red-500 font-bold" {(err.unwrap_or(""))}
                }
            }
        }
    }
}

pub fn sign_in_page(err: Option<&str>) -> Markup {
    html! {
        div id="sign-in-page" {
            div class="h-56" {}
            form
                id="login-form"
                hx-post="/login"
                hx-target="#sign-in-page"
                class="w-96 mx-auto"
            {
                div class="flex flex-col justify-center items-stretch" {
                    label class="w-full mb-2 font-bold"{ "Username" }
                    input id="username" class="bg-zinc-800 px-4 py-2 rounded-lg" name="username" {}
                    div class="h-8" {}
                    label class="w-full mb-2 font-bold" { "Password" }
                    input id="password" class="bg-zinc-800 px-4 py-2 rounded-lg" type="pasword" name="password" {}
                    div class="h-8" {}
                    input
                        type="submit"
                        value="Log in"
                        class="px-4 py-2 bg-sky-500 rounded-lg hover:bg-sky-400"
                        {}
                }
                div class="h-8" {}
                p class="text-red-500 font-bold" {(err.unwrap_or(""))}
            }
            a href="/sign-up" {
                p class="w-96 mx-auto text-gray-400" { "Sign up" }
            }
        }
    }
}

pub fn food_creator() -> Markup {
    html! {
        div {
            form
                id="food-form"
                hx-post="/create_food"
                hx-target="#status"
                hx-swap="outerHTML"
                class=""
            {
                div class="grid grid-cols-2 gap-4" {
                    label class="font-bold col-span-2" { "Food Name" }
                    input id="name" class="bg-zinc-800 px-4 py-2 rounded-lg col-span-2" name="name" {}
                    label for="calories" class="mb-2 font-bold" { "Calories per 100g" }
                    label for="protein" class="mb-2 font-bold" { "Protein per 100g" }
                    input id="calories" class="bg-zinc-800 px-4 py-2 rounded-lg" name="calories" {}
                    input id="protein" class="bg-zinc-800 px-4 py-2 rounded-lg" name="protein" {}
                    input
                        type="submit"
                        value="Create Food"
                        class="px-4 py-2 bg-sky-500 rounded-lg hover:bg-sky-400 col-span-2"
                        {}
                    p id="status" {}
                }
            }
        }
    }
}

pub fn food_searcher() -> Markup {
    html! {
        h3 class="food_title text-xl" { "Search Foods" span class="food_indicator ml-4" { "Searching..." } }
        div class="h-4" {}
        input
            class="food-search bg-zinc-800 px-4 py-2 rounded-lg"
            type="search"
            name="search-name"
            placeholder="Begin typing to search foods"
            hx-post="/search_food"
            hx-trigger="input changed delay:200ms, search-name"
            hx-target="#food-results"
            hx-indicator=".food_indicator"
        {}
        div class="h-8" {}
        table class="text-white w-full" {
            thead {
                tr {
                    th class="text-left pr-8" { "Name" }
                    th class="text-right" { "Calories (g/100g)" }
                    th class="text-right pl-4" { "Protein (g/100g)" }
                }
            }
            tbody id="food-results" {
            }
        }
    }
}
