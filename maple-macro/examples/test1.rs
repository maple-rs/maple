#![feature(proc_macro_hygiene)]
use maple_macro::view;

fn main() {
    view! {
        <test dasdasd="string">
            <div dasdasd=4.6>
                "dasdasdasdasdas"
                { 1 + 1 }
                { ... test() }
            </div>
            <div />
        </test>
    }
}
