///
/// 
/// 
/// 
pub struct Node <E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<Engine = E, ChildrenEngine = CHE>,
        CH: Renderable<Engine = CHE>
{
    item: I,
    children: CH
}

impl<E, I, CHE, CH> Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<Engine = E, ChildrenEngine = CHE>,
        CH: Renderable<Engine = CHE>
{
    pub fn new(item: I, children: CH) -> Self {
        Self {
            item,
            children
        }
    }
}

impl<E, I, CHE, CH> Renderable for Node<E, I, CHE, CH>
    where 
        E: Engine,
        CHE: Engine,
        I: RenderImplementation<Engine = E, ChildrenEngine = CHE>,
        CH: Renderable<Engine = CHE>
{
    type Engine = E;

    fn render(&self, eng: &Self::Engine) {
        self.item.render_impl(eng, &self.children)
    }
}
