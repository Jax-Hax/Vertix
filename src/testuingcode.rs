for *something that implements trait collision(&mut self)*, collision{
    trait collision()
}
for each item{
    update
}

you write the code per entity somehow, and behind the scenes it uses a function with 
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
in the prefab and then will go through and check collisio



or

keep current thing, add an event for each one and for like collisoin ytou woudl do:

for each balloon{

    state.check collision (balloon.collider), and it will look for every collision via world.get<Collider> (but use a unique id for each one to make sure it doesnt collide with itself, only check for the unique id on collides though for performance)
}


update would be:

for each balloon{
    balloon.pos.x += balloon.velocity.x etc etc
}


click gets passed in coords and runs
for each baloon {
    baloon.collider.check_collision(coords)
}
















collider{
    bounds,
    offset, (positoin + this)
    layer (like unity layers)
}
