/*
To Launch the app, run
trunk serve
and then go to
http://localhost:8080/

Tutorial spot: https://yew.rs/getting-started/build-a-sample-app

Tutorial vid: https://www.youtube.com/watch?v=lmLiMozWNGA

useful article: https://dev.to/fllstck/basic-interactions-with-yew-3pa3

*/

#![recursion_limit = "640"]

use yew::prelude::*;
use std::cmp::Ordering;
use std::io::{Read, Write};

enum Msg {
    AddOne,
    Left,
    Right,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // A couple things, like "value", are still here from the base template Yew App
    link: ComponentLink<Self>,
    value: i64,
    // This will contain our list of favorite items
    favs: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        // It seems that you can't read in from a file in Yew
//        let strings = std::fs::read_to_string("src/List.txt").unwrap();


        // So instead of reading in a file, we're doing this for now
        let strings =  "Zelda NES
        Zelda II
        A Link to the Past
        Link's Awakening
        Ocarina of Time
        Majora's Mask
        Oracle of Ages
        Oracle of Seasons
        Four Swords
        Four Swords Adventures
        The Wind Waker
        Twilight Princess
        Minish Cap
        Phantom Hourglass
        Spirit Tracks
        Skyward Sword
        A Link Between Worlds
        Breath of the Wild";

        // Put each line into a vector of favorites
        let mut favo: Vec<String> = strings.lines().map(|s| s.to_string()).collect();
        // Can we print this?
        println!("{:#?}", favo);

        // Assign favs to favo. Maybe could skip a step and assign it to what we're putting in favo
        // above, but I'm not so worried about that at the moment.
        Self {
            link,
            value: 0,
            favs: favo,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        // This is the example message. I've left it here for reference for now
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            // This occur when the left item is clicked
            Msg::Left => {
                self.favs.sort_unstable_by(|left, right| {
                    Ordering::Less
                });
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            // This occur when the right item is clicked
            Msg::Right => {
                self.favs.sort_unstable_by(|left, right| {
                    Ordering::Greater
                });
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            // But how will the program know when the sorting is done? I can't seem to grasp
            // how exactly sort_unstable_by works, I think
            // do I need to use a different sorting method for all of this, perhaps?

        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Left)>{
                // Right here I'd like to put in a string to display the "left" option
                // but I can't figure out how
                &self.favs[0]
                }</button>
                <button onclick=self.link.callback(|_| Msg::Right)>{
                // Right here I'd like to put in a string to display the "right" option
                // but I can't figure out how
                &self.favs[1]
                }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}