-- Add up migration script here
INSERT INTO apods (img_date, explanation, title, url)
SELECT to_timestamp(apod->>'date', 'YYYY-MM-DD'), apod->>'explanation', apod->>'title', apod->>'url'
FROM json_array_elements('[
    {
        "date": "2017-07-08",
        "explanation": "Similar in size to large, bright spiral galaxies in our neighborhood, IC 342 is a mere 10 million light-years distant in the long-necked, northern constellation Camelopardalis. A sprawling island universe, IC 342 would otherwise be a prominent galaxy in our night sky, but it is hidden from clear view and only glimpsed through the veil of stars, gas and dust clouds along the plane of our own Milky Way galaxy. Even though IC 342s light is dimmed by intervening cosmic clouds, this sharp telescopic image traces the galaxys own obscuring dust, blue star clusters, and glowing pink star forming regions along spiral arms that wind far from the galaxys core. IC 342 may have undergone a recent burst of star formation activity and is close enough to have gravitationally influenced the evolution of the local group of galaxies and the Milky Way.",
        "title": "Hidden Galaxy IC 342",
        "url": "https://apod.nasa.gov/apod/image/1707/ic342_rector1024s.jpg"
    },
    {
        "date": "2017-07-09",
        "explanation": "Can you find your favorite country or city? Surprisingly, on this world-wide nightscape, city lights make this task quite possible. Human-made lights highlight particularly developed or populated areas of the Earths surface, including the seaboards of Europe, the eastern United States, and Japan. Many large cities are located near rivers or oceans so that they can exchange goods cheaply by boat. Particularly dark areas include the central parts of South America, Africa, Asia, and Australia. The featured composite was created from images that were collected during cloud-free periods in April and October 2012 by the Suomi-NPP satellite, from a polar orbit about 824 kilometers above the surface, using its Visible Infrared Imaging Radiometer Suite (VIIRS).",
        "title": "Earth at Night",
        "url": "https://apod.nasa.gov/apod/image/1707/EarthAtNight_SuomiNPP_1080.jpg"
    },
    {
        "date": "2017-07-10",
        "explanation": "Whats happening around the center of this spiral galaxy? Seen in total, NGC 1512 appears to be a barred spiral galaxy -- a type of spiral that has a straight bar of stars across its center. This bar crosses an inner ring, though, a ring not seen as it surrounds the pictured region. Featured in this Hubble Space Telescope image is a nuclear ring -- one that surrounds the nucleus of the spiral. The two rings are connected not only by a bar of bright stars but by dark lanes of dust. Inside of this nuclearring, dust continues to spiral right into the very center -- possibly the location of a large black hole. The rings are bright with newly formed stars.",
        "title": "Spiral Galaxy NGC 1512: The Nuclear Ring",
        "url": "https://apod.nasa.gov/apod/image/1707/NGC1512_Schmidt_960.jpg"
    },
    {
        "date": "2017-07-11",
        "explanation": "Behold the largest ball of stars in our galaxy. Omega Centauri is packed with about 10 million stars, many older than our Sun and packed within a volume of only about 150 light-years in diameter. The star cluster is the largest and brightest of 200 or so known globular clusters that roam the halo of our Milky Way galaxy. Though most star clusters consist of stars with the same age and composition, the enigmatic Omega Cen exhibits the presence of different stellar populations with a spread of ages and chemical abundances. In fact, Omega Cen may be the remnant core of a small galaxy merging with the Milky Way. The featured image shows so many stars because it merged different exposures with high dynamic range (HDR) techniques. Omega Centauri, also known as NGC 5139, lies about 15,000 light-years away toward the southern constellation of the Centaurus.",
        "title": "Star Cluster Omega Centauri in HDR",
        "url": "https://apod.nasa.gov/apod/image/1707/OmegaCentauri_ODay_1080.jpg"
    }
]') AS apod