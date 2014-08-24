extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;
//use std::io::timer;
use std::rand::{task_rng, Rng};

use piston::
{
    Game,
    GameWindowSettings,
    GameIteratorSettings,
    RenderArgs,
    UpdateArgs,
    keyboard,
    KeyPressArgs
};

use graphics::
{
    Context,
    AddRectangle,
    AddColor,
    Draw,
//    RelativeTransform2d
};

pub struct App
{
    gl: Gl
}

static mut firstWave : bool = true; // used to stop sprites from disappearing by implementing two cases
static mut secondWave : bool = true;

static mut spritex : f64 = 1.0; // use two variables to make the sprites come in a continuous wave
static mut spritex2 : f64 = 1200.0;

static mut playerx : f64 = 275.0;
static mut playery : f64 = 275.0; 
static mut playerl : f64 = 50.0; // player length 
static mut playerh : f64 = 50.0; // player height
static mut playerv : f64 = 0.0; // this is the velocity of the player towards the ground

/*static mut rand0 : f64 = task_rng().gen_range(0.0, 600.0);
static mut rand1 : f64 = task_rng().gen_range(0.0, 600.0);
static mut rand2 : f64 = task_rng().gen_range(0.0, 600.0);
static mut rand3 : f64 = task_rng().gen_range(0.0, 600.0);
static mut rand4 : f64 = task_rng().gen_range(0.0, 600.0);*/

static mut rand0 : f64 = 0.0;
static mut rand1 : f64 = 0.0;
static mut rand2 : f64 = 0.0;
static mut rand3 : f64 = 0.0;
static mut rand4 : f64 = 0.0;

static mut rand5 : f64 = 0.0;
static mut rand6 : f64 = 0.0;
static mut rand7 : f64 = 0.0;
static mut rand8 : f64 = 0.0;
static mut rand9 : f64 = 0.0;

impl Game for App
{
    fn key_press(&mut self, _args: &KeyPressArgs)
    {
        if _args.key == keyboard::Up
        {
            unsafe
            {
                playerv = -5.0;
            }
        }
    }

    fn render(&mut self, args: &RenderArgs)
    {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0,0.0,0.1,0.1).draw(&mut self.gl);

        unsafe
        {
            context
                .rect(playerx, playery, playerl, playerh)
                .rgba(0.0, 0.5, 1.0, 1.0)
                .draw(&mut self.gl);
        }

        let drawSquare = | x : f64, y : f64, l : f64, h : f64 | 
        {
            // bounds are (left, right, top, bottom) of the sprites
            let bounds = ( x, x + l, y, y + h );
            let mut contacted : bool = false;
            
            unsafe
            {
                if bounds.val1() >= (playerx - 2.0) 
                {
                    if bounds.val1() <= (playerx + 2.0)
                    {
                        if bounds.val3() > playery
                        {
                            if bounds.val2() < (playery + playerh)
                            {
                                println!("contact left");
                                contacted = true;
                                context
                                    .rect(0.0, 0.0, 600.0, 600.0)
                                    .rgba(1.0, 0.0, 0.0, 1.0)
                                    .draw(&mut self.gl);
                            }
                        }
                    }
                }

                if bounds.val0() >= (playerx + playerl - 2.0) 
                {
                    if bounds .val0() <= (playerx + playerl + 2.0)
                    {
                        if bounds.val2() < (playery + playerh)
                        {
                            if bounds.val3() > playery
                            {
                                println!("contact right");
                                contacted = true;
                                context
                                    .rect(0.0, 0.0, 600.0, 600.0)
                                    .rgba(1.0, 0.0, 0.0, 1.0)
                                    .draw(&mut self.gl);
                            }
                        }
                    }
                }

                if bounds.val2() >= (playery + playerh - 2.0)
                {
                    if bounds.val2() <= (playery + playerh + 2.0)
                    {
                        if bounds.val1() > playerx
                        {
                            if bounds.val0() < (playerx + playerl)
                            {
                                println!("contact bottom");
                                contacted = true;
                                context
                                    .rect(0.0, 0.0, 600.0, 600.0)
                                    .rgba(1.0, 0.0, 0.0, 1.0)
                                    .draw(&mut self.gl);
                            }
                        }
                    }
                }

                if bounds.val3() >= (playery - 2.0)
                {
                    if bounds.val3() <= (playery + 2.0)
                    {
                        if bounds.val2() <= (playery + playerh + 2.0)
                        {
                            if bounds.val1() > playerx
                            {
                                if bounds.val0() < (playerx + playerl)
                                {
                                    println!("contact top");
                                    contacted = true;
                                    context
                                        .rect(0.0, 0.0, 600.0, 600.0)
                                        .rgba(1.0, 0.0, 0.0, 1.0)
                                        .draw(&mut self.gl);
                                }
                            }
                        }
                    }
                }
            }

            if !contacted
            {
            context
                .rect(x, y, l, h)
                .rgba(0.0, 1.0, 1.0, 1.0)
                .draw(&mut self.gl);
            }
        };

        unsafe
        {
            drawSquare(spritex + rand4, rand0, 10.0, 10.0);
            drawSquare(spritex + rand2, rand1, 10.0, 10.0);
            drawSquare(spritex + rand3, rand2, 10.0, 10.0);
            drawSquare(spritex + rand1, rand3, 10.0, 10.0);
            drawSquare(spritex + rand0, rand4, 10.0, 10.0);
            drawSquare(spritex + rand1, rand1, 10.0, 10.0);
            drawSquare(spritex + rand2, rand2, 10.0, 10.0);
            drawSquare(spritex + rand3, rand3, 10.0, 10.0);
            drawSquare(spritex2 + rand5, rand8, 10.0, 10.0);
            drawSquare(spritex2 + rand6, rand8, 10.0, 10.0);
            drawSquare(spritex2 + rand7, rand7, 10.0, 10.0);
            drawSquare(spritex2 + rand8, rand6, 10.0, 10.0);
            drawSquare(spritex2 + rand9, rand6, 10.0, 10.0);
            drawSquare(spritex2 + rand8, rand5, 10.0, 10.0);
            drawSquare(spritex2 + rand7, rand6, 10.0, 10.0);
            drawSquare(spritex2 + rand6, rand7, 10.0, 10.0);
            drawSquare(spritex2 + rand5, rand8, 10.0, 10.0);
            drawSquare(spritex2 + rand9, rand9, 10.0, 10.0);
        }
    }

    #[allow(unused_variable)]
    fn update(&mut self, args: &UpdateArgs)
    {
        unsafe
        {
            spritex -= 1.0;
            spritex2 -= 1.0;

            if spritex == 0.0
            {
                if firstWave
                {
                rand0 = task_rng().gen_range(0.0, 600.0);
                rand1 = task_rng().gen_range(0.0, 600.0);
                rand2 = task_rng().gen_range(0.0, 600.0);
                rand3 = task_rng().gen_range(0.0, 600.0);
                rand4 = task_rng().gen_range(0.0, 600.0);
                spritex = 600.0;
                firstWave = false;
                }
            }
                if !firstWave
                {
                    if spritex + rand0 <= 0.0
                    {
                        if spritex + rand1 <= 0.0
                        {
                            if spritex + rand2 <= 0.0
                            {
                                if spritex + rand3 <= 0.0
                                {
                                    if spritex + rand4 <= 0.0
                                    {
                                        rand0 = task_rng().gen_range(0.0, 600.0);
                                        rand1 = task_rng().gen_range(0.0, 600.0);
                                        rand2 = task_rng().gen_range(0.0, 600.0);
                                        rand3 = task_rng().gen_range(0.0, 600.0);
                                        rand4 = task_rng().gen_range(0.0, 600.0);
                                        spritex = 600.0;
                                    }
                                }
                            }
                        }
                    }
                }  

            if spritex2 == 1199.0
            {
                if secondWave
                {
                rand5 = task_rng().gen_range(0.0, 600.0);
                rand6 = task_rng().gen_range(0.0, 600.0);
                rand7 = task_rng().gen_range(0.0, 600.0);
                rand8 = task_rng().gen_range(0.0, 600.0);
                rand9 = task_rng().gen_range(0.0, 600.0);
                spritex = 600.0;
                secondWave = false;
                }
            }

                if !secondWave
                {
                    if spritex2 + rand5 <= 0.0
                    {
                        if spritex2 + rand6 <= 0.0
                        {
                            if spritex2 + rand7 <= 0.0
                            {
                                if spritex2 + rand8 <= 0.0
                                {
                                    if spritex2 + rand9 <= 0.0
                                    {
                                        rand5 = task_rng().gen_range(0.0, 600.0);
                                        rand6 = task_rng().gen_range(0.0, 600.0);
                                        rand7 = task_rng().gen_range(0.0, 600.0);
                                        rand8 = task_rng().gen_range(0.0, 600.0);
                                        rand9 = task_rng().gen_range(0.0, 600.0);
                                        spritex2 = 600.0;
                                    }
                                }
                            }
                        }
                    }
                }
 
            if playery >= 600.0 - playerh
            {
                    playerv = -0.5 * playerv; // bounce the player off the floor
                    playery = 600.0 - playerh; // prevent bugs from happening with increasingly tiny bounces
            }

            if playery <= 0.0
            {
                playerv = -0.2 * playerv; // bounce the player off the ceiling
                playery = 1.0; // prevent increasingly tiny bounce effect
            }

            playery += playerv; // move the player in the direction of the player's velocity
            playerv += 0.2; // accelerate the player towards the ground 
        }
    }
}

fn main()
{
    let mut window = GameWindowSDL2::new(
        GameWindowSettings
        {
            title: "YOLO".to_string(),
            size: [600, 600],
            fullscreen: false,
            exit_on_esc: true
        }
);

let game_iter_settings = GameIteratorSettings
{
    updates_per_second: 60,
    max_frames_per_second: 60
};

let mut app = App
{
    gl: Gl::new()
};

app.run(&mut window, &game_iter_settings);
}
