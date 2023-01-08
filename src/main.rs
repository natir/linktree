/* std use */

/* crate use */

/* project use */
use linktree::Linktree;

/* mod declaration */
mod linktree;
mod buttons;


fn main() {
    yew::Renderer::<linktree::Linktree>::new().render();
}
