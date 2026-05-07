use std::collections::HashMap;

pub struct Anglish {
    words: HashMap<&'static str, &'static str>,
    deep: HashMap<&'static str, &'static str>,
    reverse: HashMap<&'static str, &'static str>,
    suffixes: Vec<(&'static str, &'static str)>,
    prefixes: Vec<(&'static str, &'static str)>,
}

impl Anglish {
    pub fn new() -> Self {
        let words = Self::build_dict();
        let deep = Self::build_deep();
        let mut reverse = HashMap::new();
        for (&english, &anglish) in &words {
            reverse.entry(anglish).or_insert(english);
        }
        let suffixes = vec![
            ("ation", "ing"),
            ("ition", "ing"),
            ("tion", "ing"),
            ("sion", "ing"),
            ("ment", "ing"),
            ("ity", "ness"),
            ("ance", "ness"),
            ("ence", "ness"),
            ("ous", "y"),
            ("al", "ly"),
            ("ical", "ish"),
            ("ify", "en"),
            ("ize", "en"),
            ("able", "worthy"),
            ("ible", "some"),
            ("age", "ing"),
            ("ist", "man"),
            ("ism", "dom"),
            ("ive", "ful"),
            ("ure", "ing"),
            ("or", "er"),
            ("ate", "en"),
        ];
        let prefixes = vec![
            ("pre", "fore"),
            ("re", "again"),
            ("dis", "mis"),
            ("anti", "against"),
            ("ex", "out"),
            ("inter", "between"),
            ("super", "over"),
            ("trans", "across"),
            ("sub", "under"),
            ("de", "down"),
            ("non", "un"),
            ("pro", "for"),
            ("post", "after"),
            ("co", "with"),
            ("counter", "against"),
            ("extra", "out"),
            ("multi", "many"),
            ("semi", "half"),
            ("bi", "two"),
            ("tri", "three"),
        ];

        Self { words, deep, reverse, suffixes, prefixes }
    }

    fn build_dict() -> HashMap<&'static str, &'static str> {
        let mut d = HashMap::new();
        d.insert("abandonment", "forsaking");
        d.insert("ability", "might");
        d.insert("absence", "lack");
        d.insert("abundance", "plenty");
        d.insert("accelerate", "speed up");
        d.insert("accept", "take");
        d.insert("acceptable", "meet");
        d.insert("access", "way in");
        d.insert("accomplish", "bring about");
        d.insert("account", "reckoning");
        d.insert("accumulate", "gather");
        d.insert("accurate", "true");
        d.insert("achieve", "attain");
        d.insert("achievement", "attaining");
        d.insert("acknowledge", "admit");
        d.insert("acquire", "gain");
        d.insert("action", "doing");
        d.insert("active", "busy");
        d.insert("actual", "sooth");
        d.insert("actually", "soothly");
        d.insert("adapt", "fit");
        d.insert("additional", "extra");
        d.insert("adequate", "enough");
        d.insert("adjust", "set");
        d.insert("administration", "leadership");
        d.insert("admire", "look up to");
        d.insert("admit", "let in");
        d.insert("adopt", "take in");
        d.insert("advance", "forward move");
        d.insert("advantage", "upper hand");
        d.insert("adventure", "daring");
        d.insert("advertise", "make known");
        d.insert("advice", "rede");
        d.insert("advise", "rede");
        d.insert("affair", "matter");
        d.insert("affect", "touch");
        d.insert("affection", "fondness");
        d.insert("afford", "pay for");
        d.insert("agenda", "daylist");
        d.insert("agent", "doer");
        d.insert("aggressive", "fighty");
        d.insert("agree", "share mind");
        d.insert("agreement", "troth");
        d.insert("agriculture", "tillage");
        d.insert("aim", "goal");
        d.insert("alcohol", "strong drink");
        d.insert("alliance", "bond");
        d.insert("allocate", "set aside");
        d.insert("allow", "let");
        d.insert("alter", "change");
        d.insert("alternative", "other choice");
        d.insert("amaze", "awe");
        d.insert("ambition", "drive");
        d.insert("amount", "sum");
        d.insert("amuse", "entertain");
        d.insert("analysis", "breakdown");
        d.insert("ancient", "old");
        d.insert("anger", "wrath");
        d.insert("announce", "tell out");
        d.insert("annual", "yearly");
        d.insert("anxiety", "unrest");
        d.insert("apologize", "say sorry");
        d.insert("apparent", "clear");
        d.insert("appeal", "call");
        d.insert("appear", "show up");
        d.insert("appearance", "look");
        d.insert("appetite", "hunger");
        d.insert("application", "use");
        d.insert("apply", "put on");
        d.insert("appoint", "name");
        d.insert("appreciate", "value");
        d.insert("approach", "come near");
        d.insert("appropriate", "fitting");
        d.insert("approval", "okay");
        d.insert("approve", "okay");
        d.insert("approximately", "about");
        d.insert("architecture", "building craft");
        d.insert("argue", "bicker");
        d.insert("argument", "bickering");
        d.insert("arrange", "set up");
        d.insert("arrangement", "setup");
        d.insert("arrest", "grab");
        d.insert("arrive", "come");
        d.insert("article", "piece");
        d.insert("artificial", "manmade");
        d.insert("artist", "craftsman");
        d.insert("aspect", "side");
        d.insert("assemble", "gather");
        d.insert("assembly", "gathering");
        d.insert("assess", "weigh up");
        d.insert("assign", "give out");
        d.insert("assist", "help");
        d.insert("assistance", "help");
        d.insert("associate", "link");
        d.insert("association", "fellowship");
        d.insert("assume", "take for granted");
        d.insert("assure", "guarantee");
        d.insert("atmosphere", "air");
        d.insert("attach", "fasten");
        d.insert("attempt", "try");
        d.insert("attend", "go to");
        d.insert("attention", "heed");
        d.insert("attitude", "mindset");
        d.insert("attract", "draw");
        d.insert("attraction", "draw");
        d.insert("attribute", "quality");
        d.insert("audience", "listeners");
        d.insert("authority", "sayso");
        d.insert("automatic", "selfworking");
        d.insert("available", "at hand");
        d.insert("average", "middling");
        d.insert("avoid", "shun");
        d.insert("award", "prize");
        d.insert("aware", "witting");
        d.insert("balance", "evenness");
        d.insert("barrier", "wall");
        d.insert("base", "bottom");
        d.insert("basic", "lowest");
        d.insert("basis", "groundwork");
        d.insert("battle", "fight");
        d.insert("beauty", "fairness");
        d.insert("beverage", "drink");
        d.insert("begin", "start");
        d.insert("behavior", "bearing");
        d.insert("believe", "trow");
        d.insert("benefit", "gain");
        d.insert("bitter", "sharp");
        d.insert("blame", "fault");
        d.insert("blossom", "bloom");
        d.insert("border", "edge");
        d.insert("boundary", "mark");
        d.insert("brave", "bold");
        d.insert("bravery", "boldness");
        d.insert("brief", "short");
        d.insert("brilliant", "bright");
        d.insert("broad", "wide");
        d.insert("budget", "spending plan");
        d.insert("burden", "load");
        d.insert("cabinet", "cupboard");
        d.insert("calculate", "reckon");
        d.insert("campaign", "drive");
        d.insert("capability", "skill");
        d.insert("capable", "able");
        d.insert("capacity", "room");
        d.insert("capture", "catch");
        d.insert("career", "lifework");
        d.insert("careful", "heedful");
        d.insert("category", "class");
        d.insert("celebrate", "mark");
        d.insert("celebration", "feast");
        d.insert("central", "middle");
        d.insert("century", "hundredyear");
        d.insert("ceremony", "rite");
        d.insert("certain", "sure");
        d.insert("certainly", "surely");
        d.insert("challenge", "dare");
        d.insert("champion", "winner");
        d.insert("chance", "luck");
        d.insert("change", "shift");
        d.insert("character", "kind");
        d.insert("characteristic", "trait");
        d.insert("charge", "fee");
        d.insert("charity", "almsgiving");
        d.insert("chief", "main");
        d.insert("childhood", "youth");
        d.insert("choice", "pick");
        d.insert("circumstance", "condition");
        d.insert("citizen", "townsman");
        d.insert("city", "town");
        d.insert("civil", "townly");
        d.insert("civilization", "culture");
        d.insert("claim", "say");
        d.insert("class", "rank");
        d.insert("classic", "timeless");
        d.insert("climate", "weather");
        d.insert("clinic", "healing house");
        d.insert("coalition", "joining");
        d.insert("coast", "shore");
        d.insert("collapse", "fall in");
        d.insert("colleague", "workmate");
        d.insert("collect", "gather");
        d.insert("collection", "gathering");
        d.insert("college", "school");
        d.insert("colony", "settlement");
        d.insert("color", "hue");
        d.insert("combat", "fight");
        d.insert("combine", "blend");
        d.insert("comfort", "coziness");
        d.insert("command", "bid");
        d.insert("commander", "leader");
        d.insert("comment", "remark");
        d.insert("commerce", "trade");
        d.insert("commercial", "trade");
        d.insert("commission", "assignment");
        d.insert("commit", "bind");
        d.insert("commitment", "binding");
        d.insert("committee", "board");
        d.insert("communicate", "pass on");
        d.insert("communication", "message");
        d.insert("community", "fellowship");
        d.insert("companion", "fellow");
        d.insert("company", "firm");
        d.insert("compare", "liken");
        d.insert("comparison", "likening");
        d.insert("compete", "vie");
        d.insert("competition", "contest");
        d.insert("competitive", "vying");
        d.insert("complain", "grumble");
        d.insert("complaint", "grumble");
        d.insert("complete", "full");
        d.insert("completely", "fully");
        d.insert("complex", "tangled");
        d.insert("complicate", "tangle");
        d.insert("component", "part");
        d.insert("compose", "write");
        d.insert("composition", "writing");
        d.insert("comprehensive", "full");
        d.insert("comprise", "contain");
        d.insert("concentrate", "focus");
        d.insert("concentration", "focus");
        d.insert("concept", "idea");
        d.insert("concern", "worry");
        d.insert("concerning", "about");
        d.insert("conclude", "end");
        d.insert("conclusion", "end");
        d.insert("condition", "state");
        d.insert("conduct", "carry out");
        d.insert("conference", "meeting");
        d.insert("confidence", "trust");
        d.insert("confident", "sure");
        d.insert("confirm", "back up");
        d.insert("conflict", "fight");
        d.insert("confuse", "mix up");
        d.insert("confusion", "mixup");
        d.insert("congress", "gathering");
        d.insert("connect", "link");
        d.insert("connection", "link");
        d.insert("conscious", "awake");
        d.insert("consequence", "outcome");
        d.insert("consequently", "so");
        d.insert("conservation", "keeping");
        d.insert("consider", "think over");
        d.insert("considerable", "sizeable");
        d.insert("consideration", "thought");
        d.insert("consist", "be made up");
        d.insert("consistent", "steady");
        d.insert("constant", "steady");
        d.insert("constitute", "make up");
        d.insert("constitution", "law");
        d.insert("construct", "build");
        d.insert("construction", "building");
        d.insert("consult", "ask");
        d.insert("consume", "use up");
        d.insert("consumer", "buyer");
        d.insert("consumption", "use");
        d.insert("contact", "touch");
        d.insert("contain", "hold");
        d.insert("commence", "start");
        d.insert("commenced", "started");
        d.insert("commences", "starts");
        d.insert("commencing", "starting");
        d.insert("container", "holder");
        d.insert("contemporary", "modern");
        d.insert("content", "happy");
        d.insert("contents", "what's inside");
        d.insert("contest", "match");
        d.insert("context", "setting");
        d.insert("continue", "go on");
        d.insert("continuous", "ongoing");
        d.insert("contract", "deal");
        d.insert("contrary", "opposite");
        d.insert("contrast", "difference");
        d.insert("contribute", "give");
        d.insert("contribution", "gift");
        d.insert("control", "handle");
        d.insert("controversy", "dispute");
        d.insert("convenient", "handy");
        d.insert("convention", "custom");
        d.insert("conversation", "talk");
        d.insert("convert", "turn");
        d.insert("convince", "win over");
        d.insert("cooperate", "work together");
        d.insert("cooperation", "teamwork");
        d.insert("coordinate", "organize");
        d.insert("cope", "deal");
        d.insert("corner", "nook");
        d.insert("corporate", "joint");
        d.insert("corporation", "company");
        d.insert("correct", "right");
        d.insert("correction", "fixing");
        d.insert("correspond", "match");
        d.insert("council", "board");
        d.insert("counsel", "rede");
        d.insert("count", "tally");
        d.insert("country", "land");
        d.insert("county", "shire");
        d.insert("couple", "pair");
        d.insert("courage", "heartness");
        d.insert("course", "path");
        d.insert("cover", "lid");
        d.insert("crack", "split");
        d.insert("crash", "smash");
        d.insert("create", "make");
        d.insert("creation", "making");
        d.insert("creative", "making");
        d.insert("creature", "being");
        d.insert("credit", "trust");
        d.insert("crime", "wrongdoing");
        d.insert("criminal", "wrongdoer");
        d.insert("crisis", "turning point");
        d.insert("criteria", "benchmarks");
        d.insert("critical", "key");
        d.insert("criticism", "faultfinding");
        d.insert("criticize", "find fault");
        d.insert("crop", "harvest");
        d.insert("crucial", "key");
        d.insert("crude", "raw");
        d.insert("cruel", "mean");
        d.insert("culture", "folkways");
        d.insert("cure", "heal");
        d.insert("curious", "nosy");
        d.insert("current", "now");
        d.insert("currently", "now");
        d.insert("curriculum", "course");
        d.insert("custom", "wont");
        d.insert("customer", "buyer");
        d.insert("cycle", "wheel");
        d.insert("daily", "everyday");
        d.insert("damage", "hurt");
        d.insert("danger", "risk");
        d.insert("dangerous", "risky");
        d.insert("data", "facts");
        d.insert("database", "data bank");
        d.insert("date", "day");
        d.insert("deadline", "last day");
        d.insert("debate", "discussion");
        d.insert("debt", "owing");
        d.insert("decade", "tenyear");
        d.insert("decay", "rot");
        d.insert("deceive", "trick");
        d.insert("decent", "proper");
        d.insert("decide", "choose");
        d.insert("decision", "choice");
        d.insert("declare", "state");
        d.insert("decline", "fall");
        d.insert("decorate", "trim");
        d.insert("decrease", "lessening");
        d.insert("dedicate", "give");
        d.insert("defeat", "beat");
        d.insert("defend", "shield");
        d.insert("defense", "shield");
        d.insert("defensive", "shielding");
        d.insert("deficit", "shortfall");
        d.insert("define", "set bounds");
        d.insert("definite", "fixed");
        d.insert("definitely", "surely");
        d.insert("definition", "meaning");
        d.insert("degree", "level");
        d.insert("delay", "wait");
        d.insert("delegate", "send");
        d.insert("delete", "remove");
        d.insert("deliberate", "intentional");
        d.insert("delicate", "fine");
        d.insert("delicious", "tasty");
        d.insert("delight", "joy");
        d.insert("deliver", "hand over");
        d.insert("delivery", "handover");
        d.insert("demand", "ask");
        d.insert("democracy", "folkrule");
        d.insert("demonstrate", "show");
        d.insert("demonstration", "showing");
        d.insert("deny", "say no");
        d.insert("department", "part");
        d.insert("departure", "leave");
        d.insert("depend", "lean");
        d.insert("dependent", "leaning");
        d.insert("depict", "show");
        d.insert("deposit", "put in");
        d.insert("depress", "sadden");
        d.insert("depression", "sadness");
        d.insert("derive", "come from");
        d.insert("describe", "tell of");
        d.insert("description", "telling");
        d.insert("desert", "waste");
        d.insert("deserve", "merit");
        d.insert("design", "plan");
        d.insert("designer", "planner");
        d.insert("desire", "wish");
        d.insert("desk", "table");
        d.insert("desperate", "hopeless");
        d.insert("despite", "in spite of");
        d.insert("destination", "goal");
        d.insert("destroy", "break");
        d.insert("destruction", "breaking");
        d.insert("detail", "point");
        d.insert("detect", "spot");
        d.insert("determination", "drive");
        d.insert("determine", "find out");
        d.insert("develop", "grow");
        d.insert("development", "growth");
        d.insert("device", "gadget");
        d.insert("devote", "give");
        d.insert("dictionary", "wordbook");
        d.insert("diet", "food");
        d.insert("differ", "vary");
        d.insert("difference", "gap");
        d.insert("different", "unlike");
        d.insert("difficult", "hard");
        d.insert("difficulty", "hardship");
        d.insert("digest", "break down");
        d.insert("digital", "numberbased");
        d.insert("dimension", "measure");
        d.insert("diminish", "lessen");
        d.insert("dinner", "meal");
        d.insert("direct", "straight");
        d.insert("direction", "way");
        d.insert("director", "leader");
        d.insert("disability", "unfitness");
        d.insert("disappear", "vanish");
        d.insert("disappoint", "let down");
        d.insert("disaster", "catastrophe");
        d.insert("discharge", "let go");
        d.insert("discipline", "training");
        d.insert("disclose", "reveal");
        d.insert("discount", "price cut");
        d.insert("discover", "find");
        d.insert("discovery", "finding");
        d.insert("discrimination", "bias");
        d.insert("discuss", "talk about");
        d.insert("discussion", "talk");
        d.insert("disease", "sickness");
        d.insert("disgust", "revulsion");
        d.insert("dish", "plate");
        d.insert("dismiss", "let go");
        d.insert("display", "show");
        d.insert("disposal", "throwing away");
        d.insert("dispose", "throw away");
        d.insert("dispute", "quarrel");
        d.insert("dissolve", "melt");
        d.insert("distance", "way off");
        d.insert("distant", "far");
        d.insert("distinct", "clear");
        d.insert("distinction", "honor");
        d.insert("distinguish", "tell apart");
        d.insert("distract", "disturb");
        d.insert("distress", "anguish");
        d.insert("distribute", "deal out");
        d.insert("distribution", "dealing out");
        d.insert("district", "area");
        d.insert("disturb", "bother");
        d.insert("diverse", "various");
        d.insert("diversity", "variety");
        d.insert("divide", "split");
        d.insert("divine", "godly");
        d.insert("division", "split");
        d.insert("divorce", "split");
        d.insert("document", "writing");
        d.insert("documentation", "writings");
        d.insert("domestic", "home");
        d.insert("dominate", "rule");
        d.insert("donation", "gift");
        d.insert("dose", "amount");
        d.insert("double", "twofold");
        d.insert("doubt", "uncertainty");
        d.insert("dozen", "twelve");
        d.insert("draft", "sketch");
        d.insert("drag", "pull");
        d.insert("drama", "play");
        d.insert("dramatic", "striking");
        d.insert("drastic", "extreme");
        d.insert("duration", "length");
        d.insert("dynamic", "lively");
        d.insert("eager", "keen");
        d.insert("economy", "sparing");
        d.insert("edge", "rim");
        d.insert("edition", "printing");
        d.insert("editor", "redactor");
        d.insert("educate", "teach");
        d.insert("education", "teaching");
        d.insert("educational", "teaching");
        d.insert("effect", "result");
        d.insert("effective", "working");
        d.insert("efficiency", "efficiency");
        d.insert("efficient", "smooth");
        d.insert("effort", "try");
        d.insert("elaborate", "detailed");
        d.insert("election", "vote");
        d.insert("electric", "powerdriven");
        d.insert("element", "part");
        d.insert("elementary", "basic");
        d.insert("elevate", "lift");
        d.insert("eliminate", "remove");
        d.insert("embarrass", "shame");
        d.insert("emerge", "come out");
        d.insert("emergency", "dire need");
        d.insert("emission", "giving off");
        d.insert("emotion", "feeling");
        d.insert("emotional", "feeling");
        d.insert("emphasis", "stress");
        d.insert("emphasize", "stress");
        d.insert("empire", "kingdom");
        d.insert("employ", "hire");
        d.insert("employee", "worker");
        d.insert("employer", "boss");
        d.insert("employment", "work");
        d.insert("enable", "make able");
        d.insert("encounter", "meet");
        d.insert("encourage", "hearten");
        d.insert("encouragement", "heartening");
        d.insert("enemy", "foe");
        d.insert("energy", "might");
        d.insert("enforce", "uphold");
        d.insert("engage", "take part");
        d.insert("engagement", "betrothal");
        d.insert("engine", "motor");
        d.insert("engineering", "craft");
        d.insert("enjoy", "like");
        d.insert("enjoyment", "fun");
        d.insert("enormous", "huge");
        d.insert("ensure", "make sure");
        d.insert("enter", "go into");
        d.insert("enterprise", "undertaking");
        d.insert("entertain", "amuse");
        d.insert("entertainment", "fun");
        d.insert("enthusiasm", "eagerness");
        d.insert("entire", "whole");
        d.insert("entirely", "wholly");
        d.insert("entitle", "call");
        d.insert("entrance", "way in");
        d.insert("entry", "way in");
        d.insert("environment", "surroundings");
        d.insert("episode", "event");
        d.insert("equal", "even");
        d.insert("equality", "evenness");
        d.insert("equipment", "gear");
        d.insert("equivalent", "equal");
        d.insert("era", "age");
        d.insert("error", "mistake");
        d.insert("escape", "get away");
        d.insert("especially", "specially");
        d.insert("essay", "writing");
        d.insert("essential", "key");
        d.insert("establish", "set up");
        d.insert("establishment", "setup");
        d.insert("estate", "property");
        d.insert("estimate", "rough guess");
        d.insert("evaluate", "weigh up");
        d.insert("evaluation", "weighing up");
        d.insert("event", "happening");
        d.insert("eventually", "in the end");
        d.insert("evidence", "proof");
        d.insert("evident", "clear");
        d.insert("evil", "bad");
        d.insert("evolution", "unfolding");
        d.insert("evolve", "unfold");
        d.insert("exact", "precise");
        d.insert("exactly", "precisely");
        d.insert("exaggerate", "overstate");
        d.insert("examination", "test");
        d.insert("examine", "look at");
        d.insert("example", "byspel");
        d.insert("exceed", "go beyond");
        d.insert("excellent", "topnotch");
        d.insert("except", "save");
        d.insert("exception", "odd one out");
        d.insert("excess", "too much");
        d.insert("excessive", "too much");
        d.insert("exchange", "swap");
        d.insert("excite", "thrill");
        d.insert("excitement", "thrill");
        d.insert("exclude", "leave out");
        d.insert("exclusive", "only");
        d.insert("excuse", "sorry");
        d.insert("execute", "carry out");
        d.insert("executive", "leader");
        d.insert("exercise", "workout");
        d.insert("exhaust", "use up");
        d.insert("exhibit", "show");
        d.insert("exhibition", "show");
        d.insert("exist", "be");
        d.insert("existence", "being");
        d.insert("expand", "spread");
        d.insert("expansion", "spread");
        d.insert("expect", "look for");
        d.insert("expectation", "hope");
        d.insert("expense", "cost");
        d.insert("expensive", "costly");
        d.insert("experience", "knowhow");
        d.insert("experiment", "test");
        d.insert("expert", "master");
        d.insert("explain", "clear up");
        d.insert("explanation", "clearing up");
        d.insert("explicit", "clear");
        d.insert("exploit", "use");
        d.insert("exploration", "exploration");
        d.insert("explore", "search");
        d.insert("explosion", "burst");
        d.insert("export", "send abroad");
        d.insert("expose", "lay bare");
        d.insert("exposure", "laying bare");
        d.insert("express", "say");
        d.insert("expression", "saying");
        d.insert("extend", "stretch");
        d.insert("extension", "stretch");
        d.insert("extensive", "wide");
        d.insert("extent", "degree");
        d.insert("external", "outer");
        d.insert("extra", "more");
        d.insert("extraordinary", "remarkable");
        d.insert("extreme", "uttermost");
        d.insert("extremely", "utterly");
        d.insert("facility", "ease");
        d.insert("fact", "sooth");
        d.insert("factor", "part");
        d.insert("factory", "works");
        d.insert("faculty", "staff");
        d.insert("fail", "fall short");
        d.insert("failure", "falling short");
        d.insert("faith", "belief");
        d.insert("faithful", "true");
        d.insert("familiar", "known");
        d.insert("family", "kin");
        d.insert("famous", "wellknown");
        d.insert("fantasy", "dream");
        d.insert("fashion", "style");
        d.insert("fatigue", "weariness");
        d.insert("fault", "blame");
        d.insert("favor", "good turn");
        d.insert("favorite", "bestloved");
        d.insert("feature", "trait");
        d.insert("federal", "union");
        d.insert("female", "womanly");
        d.insert("festival", "feast");
        d.insert("fiction", "story");
        d.insert("fierce", "wild");
        d.insert("figure", "number");
        d.insert("file", "folder");
        d.insert("final", "last");
        d.insert("finally", "at last");
        d.insert("finance", "money matters");
        d.insert("financial", "money matters");
        d.insert("firm", "solid");
        d.insert("fix", "mend");
        d.insert("flavor", "taste");
        d.insert("flexible", "bendy");
        d.insert("flood", "deluge");
        d.insert("flourish", "thrive");
        d.insert("flower", "blossom");
        d.insert("focus", "center");
        d.insert("folk", "folk");
        d.insert("force", "might");
        d.insert("forecast", "prediction");
        d.insert("foreign", "outlandish");
        d.insert("forest", "woods");
        d.insert("formal", "official");
        d.insert("format", "shape");
        d.insert("formation", "forming");
        d.insert("former", "earlier");
        d.insert("formerly", "earlier");
        d.insert("formula", "rule");
        d.insert("fortune", "luck");
        d.insert("foundation", "base");
        d.insert("fraction", "bit");
        d.insert("fragment", "shard");
        d.insert("frequent", "often");
        d.insert("frequently", "often");
        d.insert("front", "fore");
        d.insert("fulfill", "fill");
        d.insert("function", "work");
        d.insert("fund", "money");
        d.insert("fundamental", "basic");
        d.insert("funding", "money");
        d.insert("future", "coming time");
        d.insert("gender", "sex");
        d.insert("general", "common");
        d.insert("generally", "mostly");
        d.insert("generate", "make");
        d.insert("generation", "age group");
        d.insert("generous", "giving");
        d.insert("genuine", "echt");
        d.insert("gesture", "motion");
        d.insert("global", "worldwide");
        d.insert("glory", "fame");
        d.insert("government", "leadership");
        d.insert("governor", "leader");
        d.insert("gradual", "slow");
        d.insert("gradually", "slowly");
        d.insert("graduate", "finisher");
        d.insert("grant", "give");
        d.insert("grateful", "thankful");
        d.insert("gratitude", "thanks");
        d.insert("grave", "serious");
        d.insert("guarantee", "warranty");
        d.insert("guard", "watch");
        d.insert("guess", "guess");
        d.insert("guest", "guest");
        d.insert("guide", "lead");
        d.insert("habitat", "home");
        d.insert("harbor", "haven");
        d.insert("harmony", "peace");
        d.insert("healthy", "hale");
        d.insert("height", "height");
        d.insert("heritage", "birthright");
        d.insert("hesitate", "waver");
        d.insert("highway", "highway");
        d.insert("history", "story");
        d.insert("honest", "truthful");
        d.insert("honesty", "truthfulness");
        d.insert("horizon", "skyline");
        d.insert("horror", "dread");
        d.insert("hospital", "sickhouse");
        d.insert("hostile", "unfriendly");
        d.insert("human", "mannish");
        d.insert("humanity", "mankind");
        d.insert("humor", "wit");
        d.insert("hypothesis", "guess");
        d.insert("idea", "thought");
        d.insert("ideal", "perfect");
        d.insert("identify", "name");
        d.insert("identity", "who you are");
        d.insert("ignore", "pay no heed");
        d.insert("illegal", "unlawful");
        d.insert("illness", "sickness");
        d.insert("illustrate", "show");
        d.insert("image", "likeness");
        d.insert("imagination", "mind's eye");
        d.insert("imagine", "dream up");
        d.insert("immediate", "straightaway");
        d.insert("immediately", "right away");
        d.insert("immense", "vast");
        d.insert("immigrant", "incomer");
        d.insert("immune", "safe");
        d.insert("impact", "hit");
        d.insert("implement", "carry out");
        d.insert("implication", "undertone");
        d.insert("imply", "hint");
        d.insert("import", "bring in");
        d.insert("importance", "weight");
        d.insert("important", "weighty");
        d.insert("impose", "force on");
        d.insert("impossible", "unpossible");
        d.insert("impress", "move");
        d.insert("impression", "feeling");
        d.insert("improve", "better");
        d.insert("improvement", "bettering");
        d.insert("impulse", "urge");
        d.insert("incident", "event");
        d.insert("include", "take in");
        d.insert("including", "with");
        d.insert("income", "takings");
        d.insert("incorporate", "include");
        d.insert("increase", "grow");
        d.insert("increasingly", "ever more");
        d.insert("incredible", "unbelievable");
        d.insert("independent", "free");
        d.insert("index", "list");
        d.insert("indicate", "point to");
        d.insert("indication", "sign");
        d.insert("individual", "single");
        d.insert("industry", "trade");
        d.insert("inevitable", "unavoidable");
        d.insert("infant", "baby");
        d.insert("infection", "taint");
        d.insert("inflation", "rising prices");
        d.insert("influence", "pull");
        d.insert("inform", "tell");
        d.insert("information", "tidings");
        d.insert("ingredient", "part");
        d.insert("inhabitant", "dweller");
        d.insert("inherit", "come into");
        d.insert("initial", "first");
        d.insert("initially", "at first");
        d.insert("initiate", "start");
        d.insert("initiative", "drive");
        d.insert("injure", "hurt");
        d.insert("injury", "hurt");
        d.insert("innovation", "new thing");
        d.insert("inquiry", "asking");
        d.insert("insect", "bug");
        d.insert("insert", "put in");
        d.insert("insist", "stand firm");
        d.insert("inspect", "look over");
        d.insert("inspection", "lookover");
        d.insert("inspire", "lift up");
        d.insert("install", "set up");
        d.insert("instance", "example");
        d.insert("instant", "blink");
        d.insert("institute", "school");
        d.insert("institution", "establishment");
        d.insert("instruction", "teaching");
        d.insert("instrument", "tool");
        d.insert("insurance", "cover");
        d.insert("intellectual", "thoughtful");
        d.insert("intelligence", "wit");
        d.insert("intelligent", "bright");
        d.insert("intend", "mean");
        d.insert("intense", "strong");
        d.insert("intensity", "strength");
        d.insert("intention", "aim");
        d.insert("interact", "talk");
        d.insert("interaction", "talk");
        d.insert("interest", "appeal");
        d.insert("interesting", "appealing");
        d.insert("interfere", "meddle");
        d.insert("interior", "inside");
        d.insert("internal", "inner");
        d.insert("international", "worldwide");
        d.insert("interpret", "overset");
        d.insert("interpretation", "oversetting");
        d.insert("interrupt", "cut in");
        d.insert("interval", "gap");
        d.insert("intervention", "stepin");
        d.insert("interview", "talk");
        d.insert("intimate", "close");
        d.insert("introduce", "bring in");
        d.insert("introduction", "bringing in");
        d.insert("invade", "march into");
        d.insert("invasion", "marching in");
        d.insert("invent", "make up");
        d.insert("invention", "making up");
        d.insert("invest", "put money in");
        d.insert("investigate", "look into");
        d.insert("investigation", "look into");
        d.insert("investment", "money put in");
        d.insert("invite", "bid");
        d.insert("involve", "wrap up in");
        d.insert("involvement", "part");
        d.insert("isolate", "cut off");
        d.insert("isolation", "cutting off");
        d.insert("issue", "matter");
        d.insert("item", "thing");
        d.insert("jealous", "greeneyed");
        d.insert("job", "work");
        d.insert("join", "link");
        d.insert("journal", "daybook");
        d.insert("journey", "trip");
        d.insert("judge", "deemster");
        d.insert("judgment", "deeming");
        d.insert("junior", "younger");
        d.insert("jury", "panel");
        d.insert("justice", "rightfulness");
        d.insert("justify", "defend");
        d.insert("keen", "sharp");
        d.insert("labor", "work");
        d.insert("lack", "lack");
        d.insert("landscape", "landskip");
        d.insert("language", "tongue");
        d.insert("large", "big");
        d.insert("largely", "mostly");
        d.insert("lawyer", "lawman");
        d.insert("lecture", "talk");
        d.insert("legal", "lawful");
        d.insert("legend", "tale");
        d.insert("legislation", "lawmaking");
        d.insert("legitimate", "lawful");
        d.insert("leisure", "free time");
        d.insert("lesson", "learning");
        d.insert("level", "even");
        d.insert("liability", "responsibility");
        d.insert("liberal", "free");
        d.insert("liberty", "freedom");
        d.insert("library", "bookhoard");
        d.insert("license", "permit");
        d.insert("lifestyle", "way of life");
        d.insert("likely", "likely");
        d.insert("limit", "bound");
        d.insert("limitation", "bound");
        d.insert("limited", "small");
        d.insert("liquid", "fluid");
        d.insert("literacy", "reading skill");
        d.insert("literary", "bookly");
        d.insert("literature", "booklore");
        d.insert("local", "nearby");
        d.insert("locate", "find");
        d.insert("location", "spot");
        d.insert("logic", "reasoncraft");
        d.insert("logical", "reasoned");
        d.insert("luxury", "wealth");
        d.insert("machine", "engine");
        d.insert("machinery", "engines");
        d.insert("magazine", "booklet");
        d.insert("magic", "witchcraft");
        d.insert("major", "main");
        d.insert("majority", "most");
        d.insert("male", "manly");
        d.insert("manage", "handle");
        d.insert("management", "handling");
        d.insert("manager", "handler");
        d.insert("manner", "way");
        d.insert("manufacture", "make");
        d.insert("manufacturing", "making");
        d.insert("margin", "edge");
        d.insert("marketing", "selling");
        d.insert("marriage", "wedlock");
        d.insert("massive", "huge");
        d.insert("material", "stuff");
        d.insert("matter", "thing");
        d.insert("mature", "ripe");
        d.insert("maximum", "most");
        d.insert("mayor", "town leader");
        d.insert("measure", "mete");
        d.insert("measurement", "meting");
        d.insert("medical", "healing");
        d.insert("medicine", "drug");
        d.insert("medium", "middle");
        d.insert("memory", "mind");
        d.insert("mental", "mindly");
        d.insert("mention", "name");
        d.insert("merchant", "trader");
        d.insert("mercy", "kindness");
        d.insert("merely", "only");
        d.insert("message", "word");
        d.insert("method", "way");
        d.insert("migration", "moving");
        d.insert("military", "warrelated");
        d.insert("million", "thousand thousand");
        d.insert("minimum", "least");
        d.insert("ministry", "department");
        d.insert("minor", "small");
        d.insert("minority", "smaller part");
        d.insert("miracle", "wonder");
        d.insert("miserable", "wretched");
        d.insert("misery", "woe");
        d.insert("mission", "task");
        d.insert("mistake", "mixup");
        d.insert("mixture", "blend");
        d.insert("mobile", "movable");
        d.insert("mode", "way");
        d.insert("model", "example");
        d.insert("moderate", "mild");
        d.insert("modern", "new");
        d.insert("modest", "humble");
        d.insert("modify", "change");
        d.insert("moment", "blink");
        d.insert("monitor", "watch");
        d.insert("monument", "landmark");
        d.insert("moral", "right");
        d.insert("mortal", "deathly");
        d.insert("motion", "move");
        d.insert("motivate", "drive");
        d.insert("motivation", "drive");
        d.insert("mountain", "fell");
        d.insert("movement", "stirring");
        d.insert("multiple", "many");
        d.insert("multiply", "manifold");
        d.insert("murder", "kill");
        d.insert("muscle", "sinew");
        d.insert("museum", "show house");
        d.insert("music", "songcraft");
        d.insert("musical", "songful");
        d.insert("mutual", "shared");
        d.insert("mysterious", "riddling");
        d.insert("mystery", "riddle");
        d.insert("myth", "tale");
        d.insert("nation", "folk");
        d.insert("national", "folkish");
        d.insert("native", "inborn");
        d.insert("natural", "kindly");
        d.insert("nature", "kind");
        d.insert("necessary", "needful");
        d.insert("necessity", "need");
        d.insert("negative", "bad");
        d.insert("neglect", "forget");
        d.insert("negotiate", "bargain");
        d.insert("negotiation", "bargaining");
        d.insert("nervous", "nervous");
        d.insert("neutral", "neither side");
        d.insert("nominate", "name");
        d.insert("normal", "usual");
        d.insert("normally", "usually");
        d.insert("notice", "heed");
        d.insert("notify", "tell");
        d.insert("notion", "idea");
        d.insert("novel", "new");
        d.insert("nuclear", "kernel");
        d.insert("number", "tally");
        d.insert("numerous", "many");
        d.insert("object", "thing");
        d.insert("objective", "goal");
        d.insert("obligation", "duty");
        d.insert("observation", "watching");
        d.insert("observe", "watch");
        d.insert("observer", "watcher");
        d.insert("obstacle", "hurdle");
        d.insert("obtain", "get");
        d.insert("obvious", "clear");
        d.insert("obviously", "clearly");
        d.insert("occasion", "time");
        d.insert("occasionally", "sometimes");
        d.insert("occupy", "fill");
        d.insert("occur", "come to pass");
        d.insert("ocean", "sea");
        d.insert("offense", "crime");
        d.insert("offensive", "insulting");
        d.insert("offer", "bid");
        d.insert("office", "room");
        d.insert("officer", "officeholder");
        d.insert("official", "officeholder");
        d.insert("operate", "work");
        d.insert("operation", "work");
        d.insert("operator", "worker");
        d.insert("opinion", "thinking");
        d.insert("opponent", "foe");
        d.insert("opportunity", "chance");
        d.insert("oppose", "withstand");
        d.insert("opposite", "other side");
        d.insert("opposition", "resistance");
        d.insert("option", "choice");
        d.insert("oral", "spoken");
        d.insert("organ", "body part");
        d.insert("organic", "natural");
        d.insert("organization", "group");
        d.insert("organize", "arrange");
        d.insert("orientation", "direction");
        d.insert("origin", "source");
        d.insert("original", "first");
        d.insert("originally", "at first");
        d.insert("outcome", "outcome");
        d.insert("overcome", "overcome");
        d.insert("overseas", "overseas");
        d.insert("pain", "ache");
        d.insert("palace", "hall");
        d.insert("panic", "fear");
        d.insert("paragraph", "section");
        d.insert("parallel", "alongside");
        d.insert("parliament", "witan");
        d.insert("part", "deal");
        d.insert("participant", "taker part");
        d.insert("participate", "take part");
        d.insert("particular", "special");
        d.insert("particularly", "especially");
        d.insert("partly", "partly");
        d.insert("partner", "mate");
        d.insert("party", "gathering");
        d.insert("passage", "way");
        d.insert("passenger", "rider");
        d.insert("passion", "strong feeling");
        d.insert("passive", "still");
        d.insert("patience", "forbearance");
        d.insert("patient", "forbearing");
        d.insert("pattern", "sample");
        d.insert("peace", "peace");
        d.insert("peculiar", "odd");
        d.insert("penalty", "fine");
        d.insert("people", "folk");
        d.insert("perceive", "see");
        d.insert("percent", "percent");
        d.insert("perception", "view");
        d.insert("perfect", "faultless");
        d.insert("perform", "do");
        d.insert("performance", "doing");
        d.insert("perhaps", "maybe");
        d.insert("period", "while");
        d.insert("permanent", "lasting");
        d.insert("permission", "leave");
        d.insert("permit", "let");
        d.insert("persist", "keep on");
        d.insert("person", "body");
        d.insert("personal", "own");
        d.insert("personality", "selfhood");
        d.insert("personnel", "staff");
        d.insert("perspective", "viewpoint");
        d.insert("persuade", "talk into");
        d.insert("phase", "stage");
        d.insert("phenomenon", "occurrence");
        d.insert("philosophy", "wisdomlore");
        d.insert("phrase", "saying");
        d.insert("physical", "bodily");
        d.insert("physician", "doctor");
        d.insert("physics", "naturelore");
        d.insert("pilot", "flyer");
        d.insert("place", "stead");
        d.insert("planet", "wanderstar");
        d.insert("plastic", "plastic");
        d.insert("platform", "stage");
        d.insert("plead", "beg");
        d.insert("pleasant", "nice");
        d.insert("plenty", "plenty");
        d.insert("pocket", "pocket");
        d.insert("poetry", "verse");
        d.insert("police", "watch");
        d.insert("policy", "way");
        d.insert("polite", "wellmannered");
        d.insert("political", "folkly");
        d.insert("politician", "lawmaker");
        d.insert("politics", "folkly matters");
        d.insert("pollution", "foulness");
        d.insert("poor", "arm");
        d.insert("popular", "beloved");
        d.insert("popularity", "belovedness");
        d.insert("population", "folkdome");
        d.insert("port", "haven");
        d.insert("portable", "carryable");
        d.insert("portion", "share");
        d.insert("portrait", "likeness");
        d.insert("position", "stead");
        d.insert("positive", "sure");
        d.insert("possess", "own");
        d.insert("possession", "ownership");
        d.insert("possibility", "likelihood");
        d.insert("possible", "may be");
        d.insert("possibly", "maybe");
        d.insert("potential", "likely");
        d.insert("poverty", "armness");
        d.insert("power", "might");
        d.insert("powerful", "mighty");
        d.insert("practical", "handy");
        d.insert("practice", "training");
        d.insert("praise", "praise");
        d.insert("precaution", "forethought");
        d.insert("precede", "come before");
        d.insert("precious", "dear");
        d.insert("precise", "exact");
        d.insert("precisely", "exactly");
        d.insert("predict", "foretell");
        d.insert("prediction", "foretelling");
        d.insert("prefer", "like better");
        d.insert("preference", "liking");
        d.insert("pregnancy", "breeding");
        d.insert("pregnant", "with child");
        d.insert("prejudice", "bias");
        d.insert("preliminary", "first");
        d.insert("premier", "first");
        d.insert("premise", "ground");
        d.insert("premium", "extra");
        d.insert("preparation", "readying");
        d.insert("prepare", "ready");
        d.insert("prescription", "script");
        d.insert("presence", "being there");
        d.insert("present", "now");
        d.insert("presentation", "talk");
        d.insert("preservation", "keeping");
        d.insert("preserve", "keep");
        d.insert("president", "fore-sitter");
        d.insert("press", "push");
        d.insert("pressure", "push");
        d.insert("prestigious", "highly thought of");
        d.insert("presumably", "likely");
        d.insert("pretend", "make believe");
        d.insert("pretty", "fair");
        d.insert("prevail", "win");
        d.insert("prevent", "stop");
        d.insert("prevention", "stopping");
        d.insert("previous", "before");
        d.insert("previously", "before");
        d.insert("price", "cost");
        d.insert("primarily", "mainly");
        d.insert("primary", "first");
        d.insert("prime", "first");
        d.insert("principal", "main");
        d.insert("principle", "grounding");
        d.insert("print", "stamp");
        d.insert("prior", "earlier");
        d.insert("priority", "first thing");
        d.insert("prison", "jail");
        d.insert("prisoner", "jailbird");
        d.insert("privacy", "privacy");
        d.insert("private", "own");
        d.insert("privilege", "birthright");
        d.insert("probably", "likely");
        d.insert("problem", "riddle");
        d.insert("procedure", "way");
        d.insert("proceed", "go on");
        d.insert("process", "going");
        d.insert("produce", "make");
        d.insert("producer", "maker");
        d.insert("product", "ware");
        d.insert("production", "making");
        d.insert("productive", "fruitful");
        d.insert("profession", "calling");
        d.insert("professional", "skilled");
        d.insert("professor", "teacher");
        d.insert("profile", "shape");
        d.insert("profit", "gain");
        d.insert("profound", "deep");
        d.insert("program", "schedule");
        d.insert("progress", "forthgoing");
        d.insert("project", "plan");
        d.insert("prominent", "wellknown");
        d.insert("promise", "word");
        d.insert("promote", "move up");
        d.insert("promotion", "move up");
        d.insert("prompt", "quick");
        d.insert("propaganda", "spin");
        d.insert("proper", "right");
        d.insert("property", "ownership");
        d.insert("proportion", "share");
        d.insert("proposal", "bid");
        d.insert("propose", "put forth");
        d.insert("proposition", "bid");
        d.insert("prosecution", "law case");
        d.insert("prospect", "outlook");
        d.insert("protect", "shield");
        d.insert("protection", "shielding");
        d.insert("protective", "shielding");
        d.insert("protest", "speak out");
        d.insert("prove", "show");
        d.insert("provide", "give");
        d.insert("provider", "giver");
        d.insert("province", "shire");
        d.insert("provision", "giving");
        d.insert("provoke", "stir up");
        d.insert("psychological", "mindly");
        d.insert("psychology", "mindlore");
        d.insert("public", "open");
        d.insert("publication", "publishing");
        d.insert("publicity", "attention");
        d.insert("publish", "issue");
        d.insert("purchase", "buy");
        d.insert("purely", "purely");
        d.insert("purpose", "aim");
        d.insert("pursue", "chase");
        d.insert("pursuit", "chase");
        d.insert("qualification", "skill");
        d.insert("qualified", "skilled");
        d.insert("qualify", "meet the bar");
        d.insert("quality", "worth");
        d.insert("quantity", "amount");
        d.insert("quest", "search");
        d.insert("question", "asking");
        d.insert("questionnaire", "asking paper");
        d.insert("quick", "fast");
        d.insert("quickly", "fast");
        d.insert("quiet", "still");
        d.insert("quit", "stop");
        d.insert("quote", "cite");
        d.insert("radical", "drastic");
        d.insert("rage", "wrath");
        d.insert("railway", "railway");
        d.insert("range", "reach");
        d.insert("rapid", "fast");
        d.insert("rapidly", "fast");
        d.insert("rarely", "seldom");
        d.insert("ratio", "ratio");
        d.insert("rational", "reasonable");
        d.insert("react", "respond");
        d.insert("reaction", "response");
        d.insert("readily", "willingly");
        d.insert("real", "sooth");
        d.insert("realistic", "downtoearth");
        d.insert("reality", "sooth");
        d.insert("realize", "see");
        d.insert("really", "soothly");
        d.insert("reason", "ground");
        d.insert("reasonable", "sound");
        d.insert("reasonably", "fairly");
        d.insert("reasoning", "thinking");
        d.insert("recall", "call back");
        d.insert("receipt", "proof of buy");
        d.insert("receive", "get");
        d.insert("receiver", "taker");
        d.insert("recent", "new");
        d.insert("recently", "lately");
        d.insert("reception", "welcome");
        d.insert("recipe", "cooking plan");
        d.insert("recognition", "knowing again");
        d.insert("recognize", "know again");
        d.insert("recommend", "speak for");
        d.insert("recommendation", "word");
        d.insert("record", "write down");
        d.insert("recover", "get back");
        d.insert("recovery", "coming back");
        d.insert("recreation", "fun");
        d.insert("recruit", "sign up");
        d.insert("reduce", "lessen");
        d.insert("reduction", "lessening");
        d.insert("refer", "point to");
        d.insert("reference", "source");
        d.insert("reflect", "think over");
        d.insert("reflection", "thought");
        d.insert("reform", "change for better");
        d.insert("refuge", "shelter");
        d.insert("refugee", "fledgling");
        d.insert("refuse", "say no");
        d.insert("regard", "look");
        d.insert("regarding", "about");
        d.insert("regardless", "no matter");
        d.insert("regime", "rule");
        d.insert("region", "reach");
        d.insert("regional", "local");
        d.insert("register", "book");
        d.insert("regret", "rue");
        d.insert("regular", "steady");
        d.insert("regularly", "steadily");
        d.insert("regulate", "rule");
        d.insert("regulation", "rule");
        d.insert("rehabilitation", "recovery");
        d.insert("reign", "rule");
        d.insert("reinforce", "strengthen");
        d.insert("reject", "turn down");
        d.insert("relate", "tell");
        d.insert("relation", "kin");
        d.insert("relationship", "kinship");
        d.insert("relative", "kin");
        d.insert("relax", "unwind");
        d.insert("relaxation", "unwinding");
        d.insert("release", "let go");
        d.insert("relevant", "fitting");
        d.insert("reliable", "trusty");
        d.insert("relief", "ease");
        d.insert("religion", "belief");
        d.insert("religious", "godly");
        d.insert("reluctant", "unwilling");
        d.insert("rely", "lean on");
        d.insert("remain", "stay");
        d.insert("remainder", "leftovers");
        d.insert("remark", "comment");
        d.insert("remarkable", "noteworthy");
        d.insert("remedy", "cure");
        d.insert("remember", "call to mind");
        d.insert("remind", "put in mind");
        d.insert("remote", "far off");
        d.insert("remove", "take away");
        d.insert("render", "make");
        d.insert("renew", "make new");
        d.insert("rent", "rent");
        d.insert("repair", "mend");
        d.insert("repeat", "say again");
        d.insert("repeatedly", "over and over");
        d.insert("replace", "put back");
        d.insert("replacement", "standin");
        d.insert("reply", "answer");
        d.insert("report", "tell");
        d.insert("reporter", "newsman");
        d.insert("represent", "stand for");
        d.insert("representation", "standin");
        d.insert("representative", "standin");
        d.insert("reproduce", "breed");
        d.insert("republic", "commonwealth");
        d.insert("reputation", "name");
        d.insert("request", "ask");
        d.insert("require", "need");
        d.insert("requirement", "need");
        d.insert("rescue", "save");
        d.insert("research", "search");
        d.insert("researcher", "searcher");
        d.insert("resemble", "look like");
        d.insert("reservation", "holding back");
        d.insert("reserve", "hold back");
        d.insert("residence", "dwelling");
        d.insert("resident", "dweller");
        d.insert("resign", "step down");
        d.insert("resist", "withstand");
        d.insert("resistance", "withstanding");
        d.insert("resolution", "settling");
        d.insert("resolve", "settle");
        d.insert("resort", "place");
        d.insert("resource", "supply");
        d.insert("respect", "honor");
        d.insert("respectively", "in that order");
        d.insert("respond", "answer");
        d.insert("response", "answer");
        d.insert("responsibility", "answerability");
        d.insert("responsible", "answerable");
        d.insert("restaurant", "eating house");
        d.insert("restore", "give back");
        d.insert("restrain", "hold back");
        d.insert("restraint", "hold back");
        d.insert("restrict", "limit");
        d.insert("restriction", "limit");
        d.insert("result", "outcome");
        d.insert("resume", "take up again");
        d.insert("retail", "selling");
        d.insert("retain", "keep");
        d.insert("retire", "step back");
        d.insert("retirement", "stepping back");
        d.insert("retreat", "withdraw");
        d.insert("return", "go back");
        d.insert("reveal", "show");
        d.insert("revenge", "vengeance");
        d.insert("revenue", "takings");
        d.insert("reverse", "turn back");
        d.insert("review", "look over");
        d.insert("revise", "rework");
        d.insert("revival", "coming back");
        d.insert("revolution", "uprising");
        d.insert("revolutionary", "uprising");
        d.insert("reward", "prize");
        d.insert("rhetoric", "speechcraft");
        d.insert("rhythm", "beat");
        d.insert("ridiculous", "laughable");
        d.insert("rigid", "stiff");
        d.insert("riot", "uproar");
        d.insert("risk", "hazard");
        d.insert("ritual", "rite");
        d.insert("rival", "foe");
        d.insert("river", "stream");
        d.insert("role", "part");
        d.insert("romantic", "lovefilled");
        d.insert("rough", "rough");
        d.insert("route", "way");
        d.insert("routine", "daily round");
        d.insert("royal", "kingly");
        d.insert("ruin", "wreck");
        d.insert("rumor", "hearsay");
        d.insert("rural", "country");
        d.insert("sacred", "holy");
        d.insert("sacrifice", "offering");
        d.insert("saint", "holy one");
        d.insert("salary", "pay");
        d.insert("sanction", "penalty");
        d.insert("satellite", "moon");
        d.insert("satisfaction", "fullness");
        d.insert("satisfactory", "good enough");
        d.insert("satisfy", "fulfill");
        d.insert("scale", "size");
        d.insert("scandal", "shame");
        d.insert("scene", "sight");
        d.insert("scenery", "landskip");
        d.insert("schedule", "timetable");
        d.insert("scheme", "plan");
        d.insert("scholar", "learner");
        d.insert("scholarship", "learning");
        d.insert("school", "learninghouse");
        d.insert("science", "knowledge");
        d.insert("scientific", "knowledgebased");
        d.insert("scientist", "knowledgeman");
        d.insert("scope", "range");
        d.insert("script", "writing");
        d.insert("season", "season");
        d.insert("secret", "secret");
        d.insert("secretary", "scribe");
        d.insert("section", "part");
        d.insert("sector", "part");
        d.insert("secure", "safe");
        d.insert("security", "safety");
        d.insert("select", "choose");
        d.insert("selection", "choice");
        d.insert("senate", "upper house");
        d.insert("senior", "older");
        d.insert("sensible", "sensible");
        d.insert("sensitive", "touchy");
        d.insert("sentiment", "feeling");
        d.insert("separate", "part");
        d.insert("separation", "parting");
        d.insert("sequence", "order");
        d.insert("serious", "earnest");
        d.insert("seriously", "earnestly");
        d.insert("servant", "server");
        d.insert("service", "help");
        d.insert("session", "sitting");
        d.insert("settlement", "settlement");
        d.insert("several", "many");
        d.insert("severe", "harsh");
        d.insert("sexual", "sex");
        d.insert("shelter", "cover");
        d.insert("signal", "sign");
        d.insert("signature", "signing");
        d.insert("significance", "weight");
        d.insert("significant", "weighty");
        d.insert("significantly", "noticeably");
        d.insert("silence", "stillness");
        d.insert("silent", "still");
        d.insert("similar", "alike");
        d.insert("similarity", "alikeness");
        d.insert("similarly", "likewise");
        d.insert("simple", "easy");
        d.insert("simply", "just");
        d.insert("simulation", "mockup");
        d.insert("simultaneously", "at the same time");
        d.insert("sincere", "heartfelt");
        d.insert("situation", "state");
        d.insert("skill", "craft");
        d.insert("skilled", "crafty");
        d.insert("slave", "thrall");
        d.insert("slavery", "thralldom");
        d.insert("social", "fellowly");
        d.insert("society", "fellowship");
        d.insert("software", "software");
        d.insert("solar", "sun");
        d.insert("soldier", "fighter");
        d.insert("sole", "only");
        d.insert("solely", "only");
        d.insert("solution", "answer");
        d.insert("sophisticated", "advanced");
        d.insert("soul", "soul");
        d.insert("source", "spring");
        d.insert("sovereign", "overlord");
        d.insert("space", "room");
        d.insert("special", "sundry");
        d.insert("specialist", "expert");
        d.insert("species", "kind");
        d.insert("specific", "certain");
        d.insert("specifically", "exactly");
        d.insert("specify", "name");
        d.insert("spectacular", "breathtaking");
        d.insert("speech", "speech");
        d.insert("speed", "speed");
        d.insert("sphere", "ball");
        d.insert("spirit", "ghost");
        d.insert("spiritual", "ghostly");
        d.insert("sponsor", "backer");
        d.insert("sport", "sport");
        d.insert("stable", "steady");
        d.insert("standard", "rule");
        d.insert("statement", "saying");
        d.insert("station", "stop");
        d.insert("statistic", "number");
        d.insert("status", "standing");
        d.insert("stomach", "belly");
        d.insert("storage", "storing");
        d.insert("store", "shop");
        d.insert("stranger", "weirdling");
        d.insert("strategy", "plan");
        d.insert("strength", "strength");
        d.insert("strengthen", "strengthen");
        d.insert("stress", "strain");
        d.insert("strict", "firm");
        d.insert("structure", "building");
        d.insert("struggle", "fight");
        d.insert("student", "learner");
        d.insert("studio", "workshop");
        d.insert("study", "learning");
        d.insert("style", "way");
        d.insert("subject", "topic");
        d.insert("subsequent", "later");
        d.insert("subsequently", "later on");
        d.insert("subsidy", "grant");
        d.insert("substance", "stuff");
        d.insert("substantial", "big");
        d.insert("substitute", "standin");
        d.insert("subtract", "take away");
        d.insert("suburb", "outskirts");
        d.insert("succeed", "do well");
        d.insert("success", "good outcome");
        d.insert("successful", "victorious");
        d.insert("successfully", "well");
        d.insert("succession", "line");
        d.insert("suffer", "endure");
        d.insert("suffering", "enduring");
        d.insert("sufficient", "enough");
        d.insert("suggest", "hint");
        d.insert("suggestion", "hint");
        d.insert("suitable", "fitting");
        d.insert("summary", "short form");
        d.insert("summit", "top");
        d.insert("super", "great");
        d.insert("superior", "better");
        d.insert("supermarket", "big shop");
        d.insert("supervise", "oversee");
        d.insert("supervision", "oversight");
        d.insert("supplement", "extra");
        d.insert("supply", "give");
        d.insert("support", "back up");
        d.insert("supporter", "backer");
        d.insert("suppose", "guess");
        d.insert("supposedly", "socalled");
        d.insert("suppress", "put down");
        d.insert("supreme", "highest");
        d.insert("surface", "top");
        d.insert("surgery", "cutting");
        d.insert("surprise", "startle");
        d.insert("surprising", "startling");
        d.insert("surprisingly", "startlingly");
        d.insert("surrender", "give up");
        d.insert("surround", "hem in");
        d.insert("survey", "look over");
        d.insert("survival", "living through");
        d.insert("survive", "live through");
        d.insert("survivor", "living through");
        d.insert("suspect", "distrust");
        d.insert("suspend", "hang");
        d.insert("suspicion", "distrust");
        d.insert("suspicious", "distrustful");
        d.insert("sustain", "keep up");
        d.insert("sustainable", "lasting");
        d.insert("symbol", "token");
        d.insert("sympathy", "fellow feeling");
        d.insert("symptom", "sign");
        d.insert("system", "way");
        d.insert("tactic", "ploy");
        d.insert("talent", "gift");
        d.insert("target", "goal");
        d.insert("task", "job");
        d.insert("teacher", "teacher");
        d.insert("technical", "craftly");
        d.insert("technique", "craft");
        d.insert("technology", "craftlore");
        d.insert("television", "telly");
        d.insert("temperature", "warmth");
        d.insert("temporary", "whilely");
        d.insert("tempt", "lure");
        d.insert("tend", "lean");
        d.insert("tendency", "leaning");
        d.insert("term", "word");
        d.insert("terminal", "end");
        d.insert("terminate", "end");
        d.insert("terrain", "land");
        d.insert("terrible", "awful");
        d.insert("territory", "land");
        d.insert("terror", "dread");
        d.insert("test", "trial");
        d.insert("testify", "bear witness");
        d.insert("testimony", "witness");
        d.insert("text", "writing");
        d.insert("theater", "playhouse");
        d.insert("theme", "topic");
        d.insert("theology", "godlore");
        d.insert("theoretical", "ideal");
        d.insert("theory", "guesswork");
        d.insert("therapist", "healer");
        d.insert("therapy", "healing");
        d.insert("therefore", "thus");
        d.insert("thesis", "claim");
        d.insert("thorough", "throughly");
        d.insert("thoroughly", "throughly");
        d.insert("threat", "threat");
        d.insert("threaten", "threaten");
        d.insert("ticket", "ticket");
        d.insert("tissue", "cloth");
        d.insert("title", "name");
        d.insert("tolerance", "forbearance");
        d.insert("tolerate", "bear");
        d.insert("tone", "tone");
        d.insert("topic", "subject");
        d.insert("total", "whole");
        d.insert("totally", "wholly");
        d.insert("touch", "feel");
        d.insert("tourism", "travel");
        d.insert("tourist", "traveler");
        d.insert("tradition", "custom");
        d.insert("traditional", "customary");
        d.insert("traffic", "travel");
        d.insert("tragedy", "woe");
        d.insert("training", "training");
        d.insert("transaction", "deal");
        d.insert("transfer", "move");
        d.insert("transform", "change");
        d.insert("transformation", "change");
        d.insert("transition", "shift");
        d.insert("translate", "overset");
        d.insert("translation", "oversetting");
        d.insert("transmission", "sending");
        d.insert("transmit", "send");
        d.insert("transparent", "seethrough");
        d.insert("transport", "carry");
        d.insert("transportation", "carrying");
        d.insert("travel", "trip");
        d.insert("treasure", "hoard");
        d.insert("treat", "deal with");
        d.insert("treatment", "handling");
        d.insert("treaty", "bond");
        d.insert("tremendous", "huge");
        d.insert("trend", "drift");
        d.insert("trial", "testing");
        d.insert("tribe", "kin");
        d.insert("triumph", "victory");
        d.insert("trouble", "bother");
        d.insert("trust", "trust");
        d.insert("truth", "sooth");
        d.insert("tutor", "teacher");
        d.insert("type", "kind");
        d.insert("typical", "usual");
        d.insert("typically", "usually");
        d.insert("ultimate", "last");
        d.insert("ultimately", "in the end");
        d.insert("unanimous", "oneminded");
        d.insert("uncertain", "unsure");
        d.insert("uncomfortable", "uneasy");
        d.insert("unconscious", "out cold");
        d.insert("undergo", "go through");
        d.insert("understand", "understand");
        d.insert("undertake", "take on");
        d.insert("undertaking", "task");
        d.insert("unemployment", "joblessness");
        d.insert("unexpected", "unforeseen");
        d.insert("unfortunate", "unlucky");
        d.insert("unfortunately", "sadly");
        d.insert("uniform", "same");
        d.insert("union", "joining");
        d.insert("unique", "oneofakind");
        d.insert("unit", "one");
        d.insert("unite", "join");
        d.insert("unity", "oneness");
        d.insert("universal", "allencompassing");
        d.insert("universe", "all");
        d.insert("university", "highschool");
        d.insert("urban", "townly");
        d.insert("urge", "push");
        d.insert("urgent", "pressing");
        d.insert("usage", "use");
        d.insert("utility", "usefulness");
        d.insert("utilization", "use");
        d.insert("utilize", "make use of");
        d.insert("utilized", "used");
        d.insert("utilizing", "using");
        d.insert("utmost", "uttermost");
        d.insert("vacation", "holiday");
        d.insert("valid", "sound");
        d.insert("valley", "dale");
        d.insert("valuable", "worthy");
        d.insert("value", "worth");
        d.insert("variable", "changeable");
        d.insert("variation", "change");
        d.insert("variety", "kind");
        d.insert("various", "sundry");
        d.insert("vary", "change");
        d.insert("vast", "vast");
        d.insert("vehicle", "wagon");
        d.insert("venture", "undertaking");
        d.insert("venue", "place");
        d.insert("verbal", "spoken");
        d.insert("verdict", "finding");
        d.insert("verify", "check");
        d.insert("version", "form");
        d.insert("versus", "against");
        d.insert("vertical", "standing");
        d.insert("very", "truly");
        d.insert("vessel", "ship");
        d.insert("veteran", "old hand");
        d.insert("viable", "workable");
        d.insert("vice", "wickedness");
        d.insert("victim", "loser");
        d.insert("victory", "victory");
        d.insert("view", "sight");
        d.insert("viewer", "watcher");
        d.insert("village", "thorpe");
        d.insert("violate", "break");
        d.insert("violation", "breaking");
        d.insert("violence", "force");
        d.insert("violent", "forceful");
        d.insert("virtue", "goodness");
        d.insert("visible", "seeable");
        d.insert("vision", "sight");
        d.insert("visit", "come see");
        d.insert("visitor", "guest");
        d.insert("visual", "seeing");
        d.insert("vital", "lifegiving");
        d.insert("vivid", "bright");
        d.insert("vocabulary", "wordhoard");
        d.insert("volume", "amount");
        d.insert("voluntary", "freewill");
        d.insert("volunteer", "freetaker");
        d.insert("vulnerable", "weak");
        d.insert("wage", "pay");
        d.insert("warn", "warn");
        d.insert("warning", "warning");
        d.insert("weakness", "weakness");
        d.insert("wealth", "wealth");
        d.insert("wealthy", "wealthy");
        d.insert("welfare", "wellbeing");
        d.insert("widespread", "widespread");
        d.insert("width", "breadth");
        d.insert("wisdom", "wisdom");
        d.insert("withdraw", "draw back");
        d.insert("withdrawal", "drawing back");
        d.insert("workforce", "workforce");
        d.insert("worldwide", "worldwide");
        d.insert("worship", "worship");
        d.insert("zeal", "passion");
        d.insert("zone", "area");
        d
    }

    fn build_deep() -> HashMap<&'static str, &'static str> {
        let mut d = HashMap::new();
        d.insert("please", "pray");
        d.insert("commence", "fall to");
        d.insert("start", "fall to");
        d.insert("begin", "fall to");
        d.insert("use", "wield");
        d.insert("container", "cask");
        d.insert("bottle", "flask");
        d.insert("drink", "mead");
        d.insert("beverage", "mead");
        d.insert("operation", "undertaking");
        d.insert("information", "lore");
        d.insert("school", "learnstead");
        d.insert("restaurant", "tavern");
        d.insert("inn", "tavern");
        d.insert("police", "warden");
        d.insert("hospital", "leechhouse");
        d.insert("president", "headman");
        d.insert("animal", "wight");
        d.insert("decide", "ordain");
        d.insert("explain", "unfold");
        d.insert("difficult", "stiff");
        d.insert("end", "outgang");
        d.insert("example", "bysen");
        d.insert("university", "leorningshouse");
        d.insert("music", "gleecraft");
        d.insert("dictionary", "wordhoard");
        d.insert("language", "speech");
        d.insert("translator", "oversetter");
        d.insert("important", "foremost");
        d.insert("immediately", "forthwith");
        d.insert("very", "sorely");
        d.insert("sorely", "sorely");
        d.insert("beautiful", "winsome");
        d.insert("happy", "blithe");
        d.insert("sad", "dreary");
        d.insert("brave", "doughty");
        d.insert("friend", "leofman");
        d.insert("enemy", "nithing");
        d.insert("world", "middangeard");
        d.insert("computer", "think-engine");
        d.insert("program", "spell");
        d.insert("data", "writ");
        d.insert("internet", "weft");
        d.insert("website", "weftstead");
        d.insert("email", "speedwrite");
        d.insert("password", "lockword");
        d.insert("file", "scroll");
        d.insert("window", "eyethirl");
        d.insert("screen", "skineboard");
        d.insert("keyboard", "keyhoard");
        d.insert("mouse", "mus");
        d.insert("download", "downbring");
        d.insert("upload", "upbring");
        d.insert("digital", "fingerish");
        d.insert("network", "net");
        d.insert("server", "thane");
        d.insert("client", "leod");
        d.insert("city", "burg");
        d.insert("river", "ea");
        d.insert("mountain", "beorg");
        d.insert("forest", "weald");
        d.insert("government", "weald");
        d.insert("parliament", "witan");
        d.insert("army", "here");
        d.insert("navy", "sciphere");
        d.insert("captain", "hlaford");
        d.insert("doctor", "leech");
        d.insert("teacher", "lareow");
        d.insert("student", "leornere");
        d.insert("book", "bec");
        d.insert("story", "spell");
        d.insert("poetry", "leothcraft");
        d.insert("art", "craeft");
        d.insert("science", "wisdomcraeft");
        d.insert("history", "gewrit");
        d.insert("law", "ae");
        d.insert("freedom", "freodom");
        d.insert("nature", "gecynd");
        d.insert("weather", "weder");
        d.insert("summer", "sumor");
        d.insert("winter", "winter");
        d.insert("spring", "lencten");
        d.insert("autumn", "haerfest");
        d.insert("morning", "morgen");
        d.insert("evening", "aefen");
        d.insert("night", "niht");
        d.insert("day", "daeg");
        d.insert("year", "gear");
        d.insert("life", "lif");
        d.insert("death", "deaeth");
        d.insert("time", "tima");
        d.insert("sea", "sae");
        d.insert("ocean", "garsecg");
        d.insert("sun", "sunne");
        d.insert("moon", "mona");
        d.insert("star", "steorra");
        d.insert("earth", "eorðe");
        d.insert("fire", "fyr");
        d.insert("water", "waeter");
        d.insert("stone", "stan");
        d.insert("house", "hus");
        d.insert("home", "ham");
        d.insert("king", "cyning");
        d.insert("queen", "cwen");
        d.insert("lord", "hlaford");
        d.insert("lady", "hlaefdige");
        d.insert("man", "waepman");
        d.insert("woman", "wifman");
        d.insert("child", "cild");
        d.insert("father", "fæder");
        d.insert("mother", "modor");
        d.insert("brother", "broþor");
        d.insert("sister", "sweostor");
        d.insert("son", "sunu");
        d.insert("daughter", "dohtor");
        d.insert("god", "god");
        d.insert("angel", "engel");
        d.insert("devil", "deofol");
        d.insert("heaven", "heofon");
        d.insert("hell", "hel");
        d.insert("soul", "sawol");
        d.insert("body", "lichama");
        d.insert("head", "heafod");
        d.insert("hand", "hond");
        d.insert("foot", "fot");
        d.insert("eye", "eage");
        d.insert("ear", "eare");
        d.insert("mouth", "muð");
        d.insert("tongue", "tunge");
        d.insert("tooth", "toð");
        d.insert("heart", "heorte");
        d.insert("blood", "blod");
        d.insert("bone", "ban");
        d.insert("bread", "hlaf");
        d.insert("milk", "meolc");
        d.insert("meat", "flaesc");
        d.insert("fish", "fisc");
        d.insert("salt", "sealt");
        d.insert("gold", "gold");
        d.insert("silver", "seolfor");
        d.insert("iron", "iren");
        d.insert("wood", "wudu");
        d.insert("door", "duru");
        d.insert("bed", "bedd");
        d.insert("table", "bord");
        d.insert("chair", "stol");
        d.insert("name", "nama");
        d.insert("word", "word");
        d.insert("song", "sang");
        d.insert("sleep", "slaep");
        d
    }

    pub fn translate(&self, input: &str) -> String {
        self.translate_inner(input, &self.words, &self.suffixes, &self.prefixes)
    }

    pub fn translate_reverse(&self, input: &str) -> String {
        self.translate_inner(input, &self.reverse, &[], &[])
    }

    pub fn translate_deep(&self, input: &str) -> String {
        let mut merged = self.words.clone();
        for (&k, &v) in &self.deep {
            merged.insert(k, v);
        }
        self.translate_inner(input, &merged, &self.suffixes, &self.prefixes)
    }

    pub fn translate_wyrd(&self, input: &str) -> String {
        let base = self.translate_deep(input);
        let mut out = String::new();
        let mut word = String::new();
        let mut the_count = 0u64;
        for ch in base.chars() {
            if ch.is_alphanumeric() || ch == '\'' {
                word.push(ch);
            } else {
                if !word.is_empty() {
                    let lower = word.to_ascii_lowercase();
                    let replacement: Option<&str> = match lower.as_str() {
                        "the" => {
                            the_count += 1;
                            if the_count % 2 == 0 {
                                Some("ye")
                            } else {
                                None
                            }
                        }
                        "and" => Some("and eke"),
                        "very" => Some("full"),
                        "you" => Some("thou"),
                        "your" => Some("thy"),
                        "yours" => Some("thine"),
                        "are" => Some("art"),
                        "is" => Some("is"),
                        "have" => Some("hath"),
                        "has" => Some("hath"),
                        "shall" => Some("shalt"),
                        "will" => Some("wilt"),
                        "can" => Some("canst"),
                        "to" => Some("to"),
                        "not" => Some("naught"),
                        "with" => Some("mid"),
                        "through" => Some("thurh"),
                        "when" => Some("when"),
                        "where" => Some("whither"),
                        "here" => Some("hither"),
                        "there" => Some("thither"),
                        "this" => Some("this"),
                        "that" => Some("that"),
                        "these" => Some("thees"),
                        "those" => Some("thoes"),
                        "yes" => Some("yea"),
                        "no" => Some("nay"),
                        "please" => Some("prithee"),
                        "sorry" => Some("alas"),
                        "hello" => Some("hail"),
                        "goodbye" => Some("farewell"),
                        "thanks" => Some("gramercy"),
                        "thank" => Some("thank"),
                        "friend" => Some("leof"),
                        _ => None,
                    };
                    match replacement {
                        Some(r) => {
                            out.push_str(&Self::preserve_case(&word, r));
                        }
                        None => {
                            out.push_str(&word);
                        }
                    }
                    word.clear();
                }
                out.push(ch);
            }
        }
        if !word.is_empty() {
            out.push_str(&word);
        }
        // occasionally prepend "Forsooth" at the start
        if out.len() > 10 {
            if let Some(first) = out.chars().next().map(|c| c.is_uppercase()) {
                if first && !out.starts_with("Forsooth") && !out.starts_with("forsooth") {
                    let rest = out.trim_start().to_string();
                    out = format!("Forsooth, {}", rest);
                }
            }
        }
        out
    }

    fn translate_inner(
        &self,
        input: &str,
        dict: &HashMap<&'static str, &'static str>,
        suffixes: &[(&'static str, &'static str)],
        prefixes: &[(&'static str, &'static str)],
    ) -> String {
        let mut result = String::new();
        let mut raw = String::new();

        for ch in input.chars() {
            if ch.is_alphanumeric() || ch == '\'' {
                raw.push(ch);
            } else {
                if !raw.is_empty() {
                    result.push_str(&Self::translate_word(raw.as_str(), dict, suffixes, prefixes));
                    raw.clear();
                }
                result.push(ch);
            }
        }
        if !raw.is_empty() {
            result.push_str(&Self::translate_word(raw.as_str(), dict, suffixes, prefixes));
        }

        result
    }

    fn translate_word(
        raw: &str,
        dict: &HashMap<&'static str, &'static str>,
        suffixes: &[(&'static str, &'static str)],
        prefixes: &[(&'static str, &'static str)],
    ) -> String {
        let lower = raw.to_ascii_lowercase();

        if let Some(replacement) = dict.get(lower.as_str()) {
            return Self::preserve_case(raw, replacement);
        }

        for (suffix, anglish_suffix) in suffixes {
            if lower.ends_with(suffix) && lower.len() > suffix.len() + 1 {
                let stem = &lower[..lower.len() - suffix.len()];
                let new_word = format!("{}{}", stem, anglish_suffix);
                return Self::preserve_case(raw, &new_word);
            }
        }

        for (prefix, anglish_prefix) in prefixes {
            if lower.starts_with(prefix) && lower.len() > prefix.len() + 1 {
                let stem = &lower[prefix.len()..];
                let new_word = format!("{}{}", anglish_prefix, stem);
                return Self::preserve_case(raw, &new_word);
            }
        }

        raw.to_string()
    }

    fn preserve_case(original: &str, replacement: &str) -> String {
        if original.chars().all(|c| c.is_uppercase()) {
            replacement.to_uppercase()
        } else if original.chars().next().map_or(false, |c| c.is_uppercase()) {
            let mut chars = replacement.chars();
            match chars.next() {
                Some(c) => {
                    let mut s = String::new();
                    s.push(c.to_ascii_uppercase());
                    s.push_str(chars.as_str());
                    s
                }
                None => replacement.to_string(),
            }
        } else {
            replacement.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_translation() {
        let a = Anglish::new();
        assert_eq!(a.translate("information"), "tidings");
        assert_eq!(a.translate("important"), "weighty");
        assert_eq!(a.translate("English"), "English");
    }

    #[test]
    fn test_preserve_case() {
        let a = Anglish::new();
        assert_eq!(a.translate("IMPORTANT"), "WEIGHTY");
        assert_eq!(a.translate("Important"), "Weighty");
    }

    #[test]
    fn test_full_sentence() {
        let a = Anglish::new();
        let input = "This is a test of the language translator.";
        let result = a.translate(input);
        assert!(result.contains("tongue"));
    }

    #[test]
    fn test_no_replacement_for_germanic_words() {
        let a = Anglish::new();
        assert_eq!(a.translate("hello"), "hello");
        assert_eq!(a.translate("world"), "world");
    }

    #[test]
    fn test_reverse_translation() {
        let a = Anglish::new();
        assert_eq!(a.translate_reverse("tidings"), "information");
        // "sooth" could map to any English word that translates to it
        let rev = a.translate_reverse("sooth");
        assert_eq!(a.translate(&rev), "sooth");
    }

    #[test]
    fn test_reverse_identity() {
        let a = Anglish::new();
        let input = "this is a test of the language";
        let anglish = a.translate(input);
        let back = a.translate_reverse(&anglish);
        assert_eq!(input, back);
    }
}
