# CS510_RWD_HW2 "Favorite APODs"
This is my CS510 Rust Web Dev repository for the final project.

# Author
Will McIntosh

# Process

This is a brief discussion of the approach I took, decisions I had to make along the way, why I made them, and what the pros/cons/tradeoffs were.

## Goals

The primary goal of this application is to have the program populate it's own database with entries from [NASA's APOD](https://apod.nasa.gov/apod/astropix.html).

Then with the images being displayed a user can press a button to make an image their favorite. When they are on their profile they can see the images that they've favorited.

If a user is on their profile page and they are an admin they have the ability to ban other users or un-ban them.

## Populating Database

This was accomplished by calling the api using the `start_date` argument set for the first of August (the month of writing this). So that means that the only images that are on this site from starting from August 1st so as to not make the page too huge.
If an image (or APOD) already exsists in the database, when the population or seeding takes place that new APOD isn't entered in the table.
I did this by making the "Image Date" variable unique and adding a constraint just incase. Since there is a new APOD every day, that also means there's only one per day, making the image date a good candidate for uniqueness.
The function that populates the database is called `get_nasa_apods` and is called in the `lib.rs` file when the backend is ran. The results are a vector of APOD objects which are iterated through and inserted into the APOD table.

Writing api calls from the backend as opposed to from the front end was very challenging in learning how to pass the database object and not make a copy when working with futures in rust. After a lot of trial and error and help from the class chatroom I was able to solve it.

## Schema

The postgres tables there were created were the following:

### Users

Adding the is_admin and is_banned variables proved to be quite a challenge to manipulate later, more structs helped!

```
id |      email      |     password      | is_admin | is_banned
---+-----------------+-------------------+----------+-----------
 2 | second@site.com | <hashed-password> | f        | f
 1 | first@site.com  | <hashed-password> | t        | f
```

### APODs

The trick here was using the urls from the NASA site themselves, instead of trying to figure out how to store the actual imaages.

```
id |      title      |     img_date      | content  | created_on |   url    
---+-----------------+-------------------+----------+------------+----------
 2 |  sun monsters   |    2023-08-01     | a pic of | 2023-08-16 | http://
 1 |   moon beams    |    2023-08-02     | moon is  | 2023-08-16 | http://
```

### Favorites

I had to add a constraint on this table. I found that with the futures in Rust, some times hitting a "Favorite" button caused the table to get spammed. So now you can't have the same apod_id and user_id pair. I also made a function in Rust while inserting favorites to move on if there's a constrain issue. A message is also provided.

```
 id | apod_id | user_id 
----+---------+---------
  1 |       2 |       1
  7 |       3 |       1
  8 |       4 |       1
 10 |       5 |       1
```

### Comments

These were entered using the `add_comment` api endpoint, which is not protected 🤫

```
 id |               content                |          created_on           | apod_id | user_id 
----+--------------------------------------+-------------------------------+---------+---------
  1 | I didn't know sun monster's existed! | 2023-08-16 17:34:55.182856+00 |       1 |       1
  2 | Such amazing sun flares!             | 2023-08-16 17:34:55.191966+00 |       1 |       2
  3 | Such smokiness!                      | 2023-08-16 17:34:55.200177+00 |       2 |       2
```

## Starting It Up

In order to run the project, heres what you need to do:
1. Turn on Docker desktop app
2. Navigate to the `/root`. Run `Docker Compose Up` in your terminal.
3. Navigate to the `/backend`. Run `sqlx migrate run` to activate the database tables.
4. Stay in `/backend`. Run `cargo run` to start the site.
5. Navigate to the `/client`. Run `cargo run` to create the test users.
6. Manually set a sql set a users to be an admin:
    * ```sql
        UPDATE users
        SET isadmin = true
        WHERE id = 1;
      ```


# Challenges

## Incognito

I had some issues with creating a JWT token because I wasn't in incognito mode. It took me too long to figure that out but when examining the auth tokens in the inspection tool, I was able to immediately recognize what was going wrong.

## More Structs

There was an issue I had initially where I realized that I needed different User structs depending on what I was doing with their information.

One was used for logging in which only needs the password and username. Some were used for signing up which needed a password confirmation, while others needed to have information concerning their admin and banned statuses. At first I was righteously confused on how to have a single struct to rule them all but with the help of the class chat I was guided towards not doing that and adding more structs.

This wasn't just the case with the Users either, it was Favorites, and Apods, and everything needed more structs!

```rust
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserDetails {
    pub id: i32,
    pub email: String,
    pub is_admin: bool,
    pub is_banned: bool,
}

make_db_id!(UserId);

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSignup {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
```

## Seeding Test Users

Seeding the database with test users was actually really challenging. I just manually entered them in and realized that because when you manually enter them into the database table they don't go through the process of creating a JWT auth token, I can't actually log in with the users I created.

## Setting A User As Admin

I overcomplicated things and tried to create a function that would be called from the back end and allow a test user to be given admin permissions. Though this would have been really cool because in just running cargo run on the client side it would have automatically populated the user's information as an admin but it was far too complicated. The primary complication was where the `pool` database was initialized and how it would have to be called up the call stack one more level in order to be passed around and passing around the database into futures wasn't easy either. Casey suggested I just mark users as admin in the sql database and that made sense.

## Too Much At One Time

I tried changing too much stuff at one time, which seemed simple initially but caused a list of problems. What I did was jump right to making a brand new Apod struct object and basically copy everything over from the Questions object from the class project. This caused a cascading affect which needed other objects that Apods was dependent upon and eventually I realized I bit off more than I can chew.

I then went back to the main branch root, force pulled and started all over again. This time with the focus of pairing down and performing automic commits for each step. I instead of creating a new class removed the Answers object, then removed anything dependent upon that. I took one step at a time until I was ready to reform things to how I wanted them, instead of creating brand new stuff. It was much easier to use a working project and commit one step at a time.

# Stretch Goals

## Comments

Unfortunately just due to a lack of time I wasn't able to implement the creation of comments on the front end. They do exist and are displayed but they have to be entered in through an api end point. If given more time I would like to have text boxes under each image ready for input from the user.

## Registration Page
There is no register page yet, these users have to be made manually. It would be nice to have a registration page but time didn't permit.

## Hosting

I really want to host this on a website with a vcpu and a database. I've never done that before I feel that this would make for a decent full stack portfolio piece.