use gpui_component::IconName;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DogId(pub usize);

pub struct Dog {
    pub id: DogId,
    pub name: &'static str,
    pub breed: &'static str,
    pub icon: IconName,
    pub facts_markdown: &'static str,
}

pub fn all_dogs() -> &'static [Dog] {
    &DOGS
}

static DOGS: [Dog; 5] = [
    Dog {
        id: DogId(0),
        name: "Luna",
        breed: "Border Collie",
        icon: IconName::LayoutDashboard,
        facts_markdown: "# Border Collie\n\n**Origin:** Scotland  \n**Life span:** 12–15 years  \n**Weight:** 14–20 kg\n\n## Temperament\n\nExtremely **intelligent** and energetic. Border Collies consistently top dog IQ rankings and are considered the *genius* of the canine world.\n\n## Notable Traits\n\n- Can understand over **1,000 words**\n- Excel at agility, herding, and frisbee\n- Need at least 2 hours of exercise daily\n- Thrive on mental challenges and problem-solving\n\n## Famous Border Collies\n\n> **Chaser**, a Border Collie from South Carolina, learned the names of **1,022 toys** — the largest vocabulary ever documented in a non-human.\n\n*\"The workaholic of the dog world.\"*",
    },
    Dog {
        id: DogId(1),
        name: "Mochi",
        breed: "Shiba Inu",
        icon: IconName::Inbox,
        facts_markdown: "# Shiba Inu\n\n**Origin:** Japan  \n**Life span:** 13–16 years  \n**Weight:** 8–11 kg\n\n## Temperament\n\nBold, fiery, and **independent** — the Shiba Inu is the most popular companion dog in Japan and the internet's favourite meme.\n\n## Notable Traits\n\n- Known for the infamous *\"Shiba scream\"* when displeased\n- Ancient breed dating back 3,000+ years\n- Cat-like cleanliness: they groom themselves obsessively\n- Extremely loyal to their family, aloof with strangers\n\n## Cultural Impact\n\n> The Shiba Inu inspired the **Doge meme** in 2010, which later became the basis for the Dogecoin cryptocurrency.\n\n*\"Much wow. Very dignified. Such independent.\"*",
    },
    Dog {
        id: DogId(2),
        name: "Biscuit",
        breed: "Golden Retriever",
        icon: IconName::Calendar,
        facts_markdown: "# Golden Retriever\n\n**Origin:** Scotland  \n**Life span:** 10–12 years  \n**Weight:** 25–34 kg\n\n## Temperament\n\nFriendly, reliable, and **utterly joyful**. The Golden Retriever has been called the quintessential family dog — their enthusiasm for life is absolutely contagious.\n\n## Notable Traits\n\n- Third most popular dog breed in the United States\n- Natural swimmers with a water-repellent double coat\n- Consistently rank as top therapy and guide dogs\n- Mouths objects gently — bred to retrieve game undamaged\n\n## Temperament Highlight\n\n> Goldens score in the **top 4** of Stanley Coren's dog intelligence rankings and are used in search-and-rescue operations worldwide.\n\n*\"Every single day is the best day ever.\"*",
    },
    Dog {
        id: DogId(3),
        name: "Pepper",
        breed: "Dachshund",
        icon: IconName::Folder,
        facts_markdown: "# Dachshund\n\n**Origin:** Germany  \n**Life span:** 12–16 years  \n**Weight:** 3–9 kg (miniature: under 5 kg)\n\n## Temperament\n\nClever, lively, and **ferociously courageous**. Do not let the short legs fool you — Dachshunds are fearless hunters with a bark that far exceeds their stature.\n\n## Notable Traits\n\n- Name literally means *\"badger dog\"* in German\n- Available in miniature and standard sizes\n- Three coat varieties: smooth, wirehaired, longhaired\n- Bred to dig into badger burrows and flush out prey\n\n## Health Note\n\n> Due to their long spine, Dachshunds are prone to **intervertebral disc disease (IVDD)**. Ramps instead of stairs and maintaining healthy weight are essential.\n\n*\"Short legs. Big attitude. Enormous heart.\"*",
    },
    Dog {
        id: DogId(4),
        name: "Storm",
        breed: "Siberian Husky",
        icon: IconName::Settings,
        facts_markdown: "# Siberian Husky\n\n**Origin:** Siberia (Chukchi people)  \n**Life span:** 12–15 years  \n**Weight:** 16–27 kg\n\n## Temperament\n\nOutgoing, mischievous, and **incredibly vocal**. Huskies are pack animals built for endurance, and they'll make absolutely sure you know when they're bored.\n\n## Notable Traits\n\n- Can pull sleds at 100+ miles per day for multiple days\n- Eyes may be blue, brown, or **heterochromatic** (one of each)\n- Double coat for extreme cold: survives down to −60 °C\n- Do not bark — they *howl*, *woo*, and *talk*\n\n## Historical Significance\n\n> In 1925, a relay of Huskies delivered diphtheria antitoxin **674 miles** across Alaska to Nome in just 5.5 days, saving the town from an epidemic. **Balto** led the final leg.\n\n*\"I didn't chew the couch. The couch jumped onto my teeth.\"*",
    },
];
