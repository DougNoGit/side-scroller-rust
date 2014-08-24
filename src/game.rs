extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;
use std::io::timer;

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
    RelativeTransform2d
};

pub struct App
{
    gl: Gl
}

static mut spritex : f64 = 600.0;

static mut playerx : f64 = 275.0;
static mut playery : f64 = 275.0; 
static mut playerl : f64 = 50.0; // player length 
static mut playerh : f64 = 50.0; // player height
static mut playerv : f64 = 0.0; // this is the velocity of the player towards the ground

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
            let bounds = ( x, x + l, y, y - (h / 2.0) );
            let mut contacted : bool = false;
            
            unsafe
            {
                if bounds.val1() == (playerx - (playerl / 2.0)) 
                {
                    if bounds.val2() > (playery - (playerh / 2.0))
                    {
                        if bounds.val3() < (playery + (playerh / 2.0))
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
                if bounds.val0() == (playerx + (playerl / 2.0)) 
                {
                    if bounds.val2() > (playery - (playerh / 2.0))
                    {
                        if bounds.val3() < (playery + (playerh / 2.0))
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
            drawSquare(spritex - 100.0, 10.0, 10.0, 10.0);
            drawSquare(spritex, 500.0, 10.0, 10.0);
            drawSquare(spritex - 500.0, 200.0, 10.0, 10.0);
            drawSquare(spritex - 300.0, 300.0, 10.0, 10.0);
            drawSquare(spritex - 200.0, 400.0, 10.0, 10.0);
            drawSquare(spritex, 100.0, 10.0, 10.0);
        }
    }

    fn update(&mut self, args: &UpdateArgs)
    {
        unsafe
        {
            spritex -= 1.0;

            if spritex == 0.0
            {
                spritex = 1100.0;
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
            playerv += 0.1; // accelerate the player towards the ground 
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
