-- Add up migration script here
INSERT INTO apods (img_date, explanation, title, url)
SELECT to_timestamp(apod->>'date', 'YYYY-MM-DD'), apod->>'explanation', apod->>'title', apod->>'url'
FROM json_array_elements('[
    {
        "copyright": "\nMike\nWenz\n",
        "date": "2023-08-01",
        "explanation": "The monsters that live on the Sun are not like us. They are larger than the Earth and made of gas hotter than in any teapot. They have no eyes, but at times, many tentacles. They float.  Usually, they slowly change shape and just fade back onto the Sun over about a month. Sometimes, though, they suddenly explode and unleash energetic particles into the Solar System that can attack the Earth.  Pictured is a huge solar prominence imaged almost two weeks ago in the light of hydrogen. Captured by a small telescope in Gilbert, Arizona, USA, the monsteresque plume of gas was held aloft by the ever-present but ever-changing magnetic field near the surface of the Sun. Our active Sun continues to show an unusually high number of prominences, filaments, sunspots, and large active regions as solar maximum approaches in 2025.",
        "hdurl": "https://apod.nasa.gov/apod/image/2308/SunMonster_Wenz_960.jpg",
        "media_type": "image",
        "service_version": "v1",
        "title": "Monster Solar Prominence",
        "url": "https://apod.nasa.gov/apod/image/2308/SunMonster_Wenz_960.jpg"
    },
    {
        "date": "2023-08-02",
        "explanation": "Why is the Cigar Galaxy billowing red smoke?  M82, as this starburst galaxy is also known, was stirred up by a recent pass near large spiral galaxy M81.  This doesn''t fully explain the source of the red-glowing outwardly expanding gas and dust, however.  Evidence indicates that this gas and dust is being driven out by the combined emerging particle winds of many stars, together creating a galactic superwind.  The dust particles are thought to originate in M82''s interstellar medium and are actually similar in size to particles in cigar smoke.  The featured photographic mosaic highlights a specific color of red light strongly emitted by ionized hydrogen gas, showing detailed filaments of this gas and dust.  The filaments extend for over 10,000 light years. The 12-million light-year distant Cigar Galaxy is the brightest galaxy in the sky in infrared light and can be seen in visible light with a small telescope towards the constellation of the Great Bear (Ursa Major).   APOD in world languages: Arabic, Bulgarian, Catalan, Chinese (Beijing), Chinese (Taiwan), Croatian, Czech, Dutch, French, German, Hebrew, Indonesian, Japanese, Montenegrin, Polish, Russian, Serbian, Slovenian,  Spanish, Taiwanese, Turkish, and  Ukrainian",
        "hdurl": "https://apod.nasa.gov/apod/image/2308/M82_HubblePathak_8150.jpg",
        "media_type": "image",
        "service_version": "v1",
        "title": "M82: Galaxy with a Supergalactic Wind",
        "url": "https://apod.nasa.gov/apod/image/2308/M82_HubblePathak_1080.jpg"
    },
    {
        "copyright": "Launch Complex 5",
        "date": "2023-08-03",
        "explanation": "In a photo from the early hours of July 29 (UTC), a Redstone rocket and Mercury capsule are on display at Cape Canaveral Launch Complex 5. Beyond the Redstone, the 8 minute long exposure has captured the arcing launch streak of a SpaceX Falcon Heavy rocket. The Falcon''s heavy communications satellite payload, at a record setting 9 metric tons, is bound for geosynchronous orbit some 22,000 miles above planet Earth. The historic launch of a Redstone rocket carried astronaut Alan Shepard on a suborbital spaceflight in May 1961 to an altitude of about 116 miles. Near the top of the frame, this Falcon rocket''s two reusable side boosters separate and execute brief entry burns. They returned to land side by side at Canaveral''s Landing Zone 1 and 2 in the distance.",
        "hdurl": "https://apod.nasa.gov/apod/image/2308/FalconHeavyRedstoneHaskell.jpeg",
        "media_type": "image",
        "service_version": "v1",
        "title": "The Falcon and the Redstone",
        "url": "https://apod.nasa.gov/apod/image/2308/FalconHeavyRedstoneHaskell1024.jpeg"
    },
    {
        "copyright": "Gianni Tumino",
        "date": "2023-08-04",
        "explanation": "A Full Moon rose as the Sun set on August 1. Near perigee, the closest point in its almost moonthly orbit, the brighter than average lunar disk illuminated night skies around planet Earth as the second supermoon of 2023. Seen here above Ragusa, Sicily, cloud banks cast diverging shadows through the supermoonlit skies, creating dramatic lunar crepuscular rays. The next Full Moon in 2023 will also shine on an August night. Rising as the Sun sets on August 30/31, this second Full Moon in a month is known as a Blue Moon. Blue moons occur only once every 2 or 3 years because lunar phases take almost a calendar month (29.5 days) to go through a complete cycle. But August''s Blue Moon will also be near perigee, the third supermoon in 2023.",
        "hdurl": "https://apod.nasa.gov/apod/image/2308/GianniTumino_Moon_Rays_JPG_LOGO_4000pix.jpg",
        "media_type": "image",
        "service_version": "v1",
        "title": "Moonrays of August",
        "url": "https://apod.nasa.gov/apod/image/2308/GianniTumino_Moon_Rays_JPG_LOGO_1024pix.jpg"
    },
    {
        "copyright": "Dong Liang",
        "date": "2023-08-05",
        "explanation": "This pretty nebula lies some 1,500 light-years away, its shape and color in this telescopic view reminiscent of a robin''s egg. The cosmic cloud spans about 3 light-years, nestled securely within the boundaries of the southern constellation Fornax. Recognized as a planetary nebula, egg-shaped NGC 1360 doesn''t represent a beginning though. Instead it corresponds to a brief and final phase in the evolution of an aging star. In fact, visible at the center of the nebula, the central star of NGC 1360 is known to be a binary star system likely consisting of two evolved white dwarf stars, less massive but much hotter than the Sun.  Their intense and otherwise invisible ultraviolet radiation has stripped away electrons from the atoms in their mutually surrounding gaseous shroud. The predominant blue-green hue of NGC 1360 seen here is the strong emission produced as electrons recombine with doubly ionized oxygen atoms.",
        "hdurl": "https://apod.nasa.gov/apod/image/2308/ngc1360_v2.jpg",
        "media_type": "image",
        "service_version": "v1",
        "title": "NGC 1360: The Robin''s Egg Nebula",
        "url": "https://apod.nasa.gov/apod/image/2308/ngc1360_v2_1024.jpg"
    },
    {
        "date": "2023-08-06",
        "explanation": "What created this unusual space ribbon? one of the most violent explosions ever witnessed by ancient humans. Back in the year 1006 AD, light reached Earth from a stellar explosion in the constellation of the Wolf (Lupus), creating a \"guest star\" in the sky that appeared brighter than Venus and lasted for over two years. The supernova, now cataloged at SN 1006, occurred about 7,000 light years away and has left a large remnant that continues to expand and fade today.  Pictured here is a small part of that expanding supernova remnant dominated by a thin and outwardly moving shock front that heats and ionizes surrounding ambient gas. The supernova remnant SN 1006 now has a diameter of nearly 60 light years.",
        "hdurl": "https://apod.nasa.gov/apod/image/2308/SN1006_Hubble_4940.jpg",
        "media_type": "image",
        "service_version": "v1",
        "title": "SN 1006: A Supernova Ribbon from Hubble",
        "url": "https://apod.nasa.gov/apod/image/2308/SN1006_Hubble_960.jpg"
    }
]') AS apod