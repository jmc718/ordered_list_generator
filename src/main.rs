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

struct Favorites {
    favorite: String,
    val: i64,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // A couple things, like "value", are still here from the base template Yew App
    link: ComponentLink<Self>,
    value: i64,
    favs : Vec<Favorites>
}



impl Model {

    fn safeIncr(&mut self) {

        if self.value == ((self.favs.len() as i64) - 1) {
            self.value = 0;
        } else {
            self.value += 1;
        }
        // self.favs[self.safeNext() as usize].val += 1;
    }



    fn safeDecr(&mut self) {

        if self.value <= 0 {
            self.value = (self.favs.len() as i64) - 1;
        } else {
            self.value -= 1;
        }
        // self.favs[self.value as usize].val += 1;

    }



    fn safeNext(& self) -> i64 {

        if self.value >= (self.favs.len() as i64) - 1 {
            return 0;
        } else {
            return self.value + 1;
        }

    }

}



impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        // It seems that you can't read in from a file in Yew
//        let strings = std::fs::read_to_string("src/List.txt").unwrap();

        // So instead of reading in a file, we're doing this for now
        let data = vec![

            Favorites { favorite: String::from("Zelda NES"), val: 0 },
            Favorites { favorite: String::from("Zelda II"), val: 0 },
            Favorites { favorite: String::from("A Link to the Past"), val: 0 },
            Favorites { favorite: String::from("Link's Awakening"), val: 0 },
            Favorites { favorite: String::from("Ocarina of Time"), val: 0 },
            Favorites { favorite: String::from("Majora's Mask"), val: 0 },

//            (String::from("Oracle of Ages"), 0),
//            (String::from("Oracle of Seasons"), 0),
//            (String::from("Four Swords"), 0),
//            (String::from("Four Swords Adventures"), 0),
//            (String::from("The Wind Waker"), 0),
//            (String::from("Twilight Princess"), 0),
//            (String::from("Minish Cap"), 0),
//            (String::from("Phantom Hourglass"), 0),
//            (String::from("Spirit Tracks"), 0),
//            (String::from("Skyward Sword"), 0),
//            (String::from("A Link Between Worlds"), 0),
//            (String::from("Breath of the Wild"), 0),

        ];

        // Assign favs to data. Maybe could skip a step and assign it to what we're putting in favo
        // above, but I'm not so worried about that at the moment.

        Self {
            link,
            value: 0,
            favs: data
        }

    }



    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // This is the example message. I've left it here for reference for now

        match msg {

            Msg::AddOne => {
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            // This occurs when the left item is clicked
            Msg::Left => {
                self.safeDecr();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            // This occurs when the right item is clicked
            Msg::Right => {
                self.safeIncr();
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
                &self.favs[self.value as usize].favorite
                }</button>
                <button onclick=self.link.callback(|_| Msg::Right)>{
                // Right here I'd like to put in a string to display the "right" option
                // but I can't figure out how
                &self.favs[self.safeNext() as usize].favorite
                }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}



fn main() {

//    Uncomment for web build, Comment for command line
     yew::start_app::<Model>();

    let strings = std::fs::read_to_string("src/List.txt").unwrap();
    let mut favs = strings.lines().collect::<Vec<_>>();
    println!("{:#?}", favs);

    // sort things
    favs.sort_unstable_by(|left, right| loop {
        let mut input = String::new();
        print!("Do you like {} more than {}? (y/n)", left, right);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.chars().next().unwrap() {
            'y' => break Ordering::Less,
            'n' => break Ordering::Greater,
            _ => println!("Please enter y or n"),
        }
    });
    println!("{:#?}", favs);

}