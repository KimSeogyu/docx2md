use crate::__string_enum;

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub enum BorderStyle {
    Nil, //No Border
    #[default]
    None, //No Border
    Single, //Single Line Border
    Thick, //Single Line Border
    Double, //Double Line Border
    Dotted, //Dotted Line Border
    Dashed, //Dashed Line Border
    DotDash, //Dot Dash Line Border
    DotDotDash, //Dot Dot Dash Line Border
    Triple, //Triple Line Border
    ThinThickSmallGap, //Thin, Thick Line Border
    ThickThinSmallGap, //Thick, Thin Line Border
    ThinThickThinSmallGap, //Thin, Thick, Thin Line Border
    ThinThickMediumGap, //Thin, Thick Line Border
    ThickThinMediumGap, //Thick, Thin Line Border
    ThinThickThinMediumGap, //Thin, Thick, Thin Line Border
    ThinThickLargeGap, //Thin, Thick Line Border
    ThickThinLargeGap, //Thick, Thin Line Border
    ThinThickThinLargeGap, //Thin, Thick, Thin Line Border
    Wave, //Wavy Line Border
    DoubleWave, //Double Wave Line Border
    DashSmallGap, //Dashed Line Border
    DashDotStroked, //Dash Dot Strokes Line Border
    ThreeDEmboss, //3D Embossed Line Border
    ThreeDEngrave, //3D Engraved Line Border
    Outset, //Outset Line Border
    Inset, //Inset Line Border
    Apples, //Apples Art Border
    ArchedScallops, //Arched Scallops Art Border
    BabyPacifier, //Baby Pacifier Art Border
    BabyRattle, //Baby Rattle Art Border
    Balloons3Colors, //Three Color Balloons Art Border
    BalloonsHotAir, //Hot Air Balloons Art Border
    BasicBlackDashes, //Black Dash Art Border
    BasicBlackDots, //Black Dot Art Border
    BasicBlackSquares, //Black Square Art Border
    BasicThinLines, //Thin Line Art Border
    BasicWhiteDashes, //White Dash Art Border
    BasicWhiteDots, //White Dot Art Border
    BasicWhiteSquares, //White Square Art Border
    BasicWideInline, //Wide Inline Art Border
    BasicWideMidline, //Wide Midline Art Border
    BasicWideOutline, //Wide Outline Art Border
    Bats, //Bats Art Border
    Birds, //Birds Art Border
    BirdsFlight, //Birds Flying Art Border
    Cabins, //Cabin Art Border
    CakeSlice, //Cake Art Border
    CandyCorn, //Candy Corn Art Border
    CelticKnotwork, //Knot Work Art Border
    CertificateBanner, //Certificate Banner Art Border
    ChainLink, //Chain Link Art Border
    ChampagneBottle, //Champagne Bottle Art Border
    CheckedBarBlack, //Black And White Bar Art Border
    CheckedBarColor, //Color Checked Bar Art Border
    Checkered, //Checkerboard Art Border
    ChristmasTree, //Christmas Tree Art Border
    CirclesLines, //Circles And Lines Art Border
    CirclesRectangles, //Circles And Rectangles Art Border
    ClassicalWave, //Wave Art Border
    Clocks, //Clocks Art Border
    Compass, //Compass Art Border
    Confetti, //Confetti Art Border
    ConfettiGrays, //Confetti Art Border
    ConfettiOutline, //Confetti Art Border
    ConfettiStreamers, //Confetti Streamers Art Border
    ConfettiWhite, //Confetti Art Border
    CornerTriangles, //Corner Triangle Art Border
    CouponCutoutDashes, //Dashed Line Art Border
    CouponCutoutDots, //Dotted Line Art Border
    CrazyMaze, //Maze Art Border
    CreaturesButterfly, //Butterfly Art Border
    CreaturesFish, //Fish Art Border
    CreaturesInsects, //Insects Art Border
    CreaturesLadyBug, //Ladybug Art Border
    CrossStitch, //Cross-Stitch Art Border
    Cup, //Cupid Art Border
    DecoArch, //Archway Art Border
    DecoArchColor, //Color Archway Art Border
    DecoBlocks, //Blocks Art Border
    DiamondsGray, //Gray Diamond Art Border
    DoubleD, //Double D Art Border
    DoubleDiamonds, //Diamond Art Border
    Earth1, //Earth Art Border
    Earth2, //Earth Art Border
    EclipsingSquares1, //Shadowed Square Art Border
    EclipsingSquares2, //Shadowed Square Art Border
    EggsBlack, //Painted Egg Art Border
    Fans, //Fans Art Border
    Film, //Film Reel Art Border
    Firecrackers, //Firecracker Art Border
    FlowersBlockPrint, //Flowers Art Border
    FlowersDaisies, //Daisy Art Border
    FlowersModern1, //Flowers Art Border
    FlowersModern2, //Flowers Art Border
    FlowersPansy, //Pansy Art Border
    FlowersRedRose, //Red Rose Art Border
    FlowersRoses, //Roses Art Border
    FlowersTeacup, //Flowers In A Teacup Art Border
    FlowersTiny, //Small Flower Art Border
    Gems, //Gems Art Border
    GingerbreadMan, //Gingerbread Man Art Border
    Gradient, //Triangle Gradient Art Border
    Handmade1, //Handmade Art Border
    Handmade2, //Handmade Art Border
    HeartBalloon, //Heart-Shaped Balloon Art Border
    HeartGray, //Gray Heart Art Border
    Hearts, //Hearts Art Border
    HeebieJeebies, //Pattern Art Border
    Holly, //Holly Art Border
    HouseFunky, //House Art Border
    Hypnotic, //Circular Art Border
    IceCreamCones, //Ice Cream Cone Art Border
    LightBulb, //Light Bulb Art Border
    Lightning1, //Lightning Art Border
    Lightning2, //Lightning Art Border
    MapPins, //Map Pins Art Border
    MapleLeaf, //Maple Leaf Art Border
    MapleMuffins, //Muffin Art Border
    Marquee, //Marquee Art Border
    MarqueeToothed, //Marquee Art Border
    Moons, //Moon Art Border
    Mosaic, //Mosaic Art Border
    MusicNotes, //Musical Note Art Border
    Northwest, //Patterned Art Border
    Ovals, //Oval Art Border
    Packages, //Package Art Border
    PalmsBlack, //Black Palm Tree Art Border
    PalmsColor, //Color Palm Tree Art Border
    PaperClips, //Paper Clip Art Border
    Papyrus, //Papyrus Art Border
    PartyFavor, //Party Favor Art Border
    PartyGlass, //Party Glass Art Border
    Pencils, //Pencils Art Border
    People, //Character Art Border
    PeopleWaving, //Waving Character Border
    PeopleHats, //Character With Hat Art Border
    Poinsettias, //Poinsettia Art Border
    PostageStamp, //Postage Stamp Art Border
    Pumpkin1, //Pumpkin Art Border
    PushPinNote2, //Push Pin Art Border
    PushPinNote1, //Push Pin Art Border
    Pyramids, //Pyramid Art Border
    PyramidsAbove, //Pyramid Art Border
    Quadrants, //Quadrants Art Border
    Rings, //Rings Art Border
    Safari, //Safari Art Border
    Sawtooth, //Saw Tooth Art Border
    SawtoothGray, //Gray Saw Tooth Art Border
    ScaredCat, //Scared Cat Art Border
    Seattle, //Umbrella Art Border
    ShadowedSquares, //Shadowed Squares Art Border
    SharksTeeth, //Shark Tooth Art Border
    ShorebirdTracks, //Bird Tracks Art Border
    Skyrocket, //Rocket Art Border
    SnowflakeFancy, //Snowflake Art Border
    Snowflakes, //Snowflake Art Border
    Sombrero, //Sombrero Art Border
    Southwest, //Southwest-Themed Art Border
    Stars, //Stars Art Border
    StarsTop, //Stars On Top Art Border
    Stars3d, //3-D Stars Art Border
    StarsBlack, //Stars Art Border
    StarsShadowed, //Stars With Shadows Art Border
    Sun, //Sun Art Border
    Swirligig, //Whirligig Art Border
    TornPaper, //Torn Paper Art Border
    TornPaperBlack, //Black Torn Paper Art Border
    Trees, //Tree Art Border
    TriangleParty, //Triangle Art Border
    Triangles, //Triangles Art Border
    Tribal1, //Tribal Art Border One
    Tribal2, //Tribal Art Border Two
    Tribal3, //Tribal Art Border Three
    Tribal4, //Tribal Art Border Four
    Tribal5, //Tribal Art Border Five
    Tribal6, //Tribal Art Border Six
    TwistedLines1, //Twisted Lines Art Border
    TwistedLines2, //Twisted Lines Art Border
    Vine, //Vine Art Border
    Waveline, //Wavy Line Art Border
    WeavingAngles, //Weaving Angles Art Border
    WeavingBraid, //Weaving Braid Art Border
    WeavingRibbon, //Weaving Ribbon Art Border
    WeavingStrips, //Weaving Strips Art Border
    WhiteFlowers, //White Flowers Art Border
    Woodwork, //Woodwork Art Border
    XIllusions, //Crisscross Art Border
    ZanyTriangles, //Triangle Art Border
    ZigZag, //Zigzag Art Border
    ZigZagStitch, //Zigzag Stitch
}

__string_enum! {
    BorderStyle {
        Nil = "nil",
        None = "none",
        Single = "single",
        Thick = "thick",
        Double = "double",
        Dotted = "dotted",
        Dashed = "dashed",
        DotDash = "dotDash",
        DotDotDash = "dotDotDash",
        Triple = "triple",
        ThinThickSmallGap = "thinThickSmallGap",
        ThickThinSmallGap = "thickThinSmallGap",
        ThinThickThinSmallGap = "thinThickThinSmallGap",
        ThinThickMediumGap = "thinThickMediumGap",
        ThickThinMediumGap = "thickThinMediumGap",
        ThinThickThinMediumGap = "thinThickThinMediumGap",
        ThinThickLargeGap = "thinThickLargeGap",
        ThickThinLargeGap = "thickThinLargeGap",
        ThinThickThinLargeGap = "thinThickThinLargeGap",
        Wave = "wave",
        DoubleWave = "doubleWave",
        DashSmallGap = "dashSmallGap",
        DashDotStroked = "dashDotStroked",
        ThreeDEmboss = "threeDEmboss",
        ThreeDEngrave = "threeDEngrave",
        Outset = "outset",
        Inset = "inset",
        Apples = "apples",
        ArchedScallops = "archedScallops",
        BabyPacifier = "babyPacifier",
        BabyRattle = "babyRattle",
        Balloons3Colors = "balloons3Colors",
        BalloonsHotAir = "balloonsHotAir",
        BasicBlackDashes = "basicBlackDashes",
        BasicBlackDots = "basicBlackDots",
        BasicBlackSquares = "basicBlackSquares",
        BasicThinLines = "basicThinLines",
        BasicWhiteDashes = "basicWhiteDashes",
        BasicWhiteDots = "basicWhiteDots",
        BasicWhiteSquares = "basicWhiteSquares",
        BasicWideInline = "basicWideInline",
        BasicWideMidline = "basicWideMidline",
        BasicWideOutline = "basicWideOutline",
        Bats = "bats",
        Birds = "birds",
        BirdsFlight = "birdsFlight",
        Cabins = "cabins",
        CakeSlice = "cakeSlice",
        CandyCorn = "candyCorn",
        CelticKnotwork = "celticKnotwork",
        CertificateBanner = "certificateBanner",
        ChainLink = "chainLink",
        ChampagneBottle = "champagneBottle",
        CheckedBarBlack = "checkedBarBlack",
        CheckedBarColor = "checkedBarColor",
        Checkered = "checkered",
        ChristmasTree = "christmasTree",
        CirclesLines = "circlesLines",
        CirclesRectangles = "circlesRectangles",
        ClassicalWave = "classicalWave",
        Clocks = "clocks",
        Compass = "compass",
        Confetti = "confetti",
        ConfettiGrays = "confettiGrays",
        ConfettiOutline = "confettiOutline",
        ConfettiStreamers = "confettiStreamers",
        ConfettiWhite = "confettiWhite",
        CornerTriangles = "cornerTriangles",
        CouponCutoutDashes = "couponCutoutDashes",
        CouponCutoutDots = "couponCutoutDots",
        CrazyMaze = "crazyMaze",
        CreaturesButterfly = "creaturesButterfly",
        CreaturesFish = "creaturesFish",
        CreaturesInsects = "creaturesInsects",
        CreaturesLadyBug = "creaturesLadyBug",
        CrossStitch = "crossStitch",
        Cup = "cup",
        DecoArch = "decoArch",
        DecoArchColor = "decoArchColor",
        DecoBlocks = "decoBlocks",
        DiamondsGray = "diamondsGray",
        DoubleD = "doubleD",
        DoubleDiamonds = "doubleDiamonds",
        Earth1 = "earth1",
        Earth2 = "earth2",
        EclipsingSquares1 = "eclipsingSquares1",
        EclipsingSquares2 = "eclipsingSquares2",
        EggsBlack = "eggsBlack",
        Fans = "fans",
        Film = "film",
        Firecrackers = "firecrackers",
        FlowersBlockPrint = "flowersBlockPrint",
        FlowersDaisies = "flowersDaisies",
        FlowersModern1 = "flowersModern1",
        FlowersModern2 = "flowersModern2",
        FlowersPansy = "flowersPansy",
        FlowersRedRose = "flowersRedRose",
        FlowersRoses = "flowersRoses",
        FlowersTeacup = "flowersTeacup",
        FlowersTiny = "flowersTiny",
        Gems = "gems",
        GingerbreadMan = "gingerbreadMan",
        Gradient = "gradient",
        Handmade1 = "handmade1",
        Handmade2 = "handmade2",
        HeartBalloon = "heartBalloon",
        HeartGray = "heartGray",
        Hearts = "hearts",
        HeebieJeebies = "heebieJeebies",
        Holly = "holly",
        HouseFunky = "houseFunky",
        Hypnotic = "hypnotic",
        IceCreamCones = "iceCreamCones",
        LightBulb = "lightBulb",
        Lightning1 = "lightning1",
        Lightning2 = "lightning2",
        MapPins = "mapPins",
        MapleLeaf = "mapleLeaf",
        MapleMuffins = "mapleMuffins",
        Marquee = "marquee",
        MarqueeToothed = "marqueeToothed",
        Moons = "moons",
        Mosaic = "mosaic",
        MusicNotes = "musicNotes",
        Northwest = "northwest",
        Ovals = "ovals",
        Packages = "packages",
        PalmsBlack = "palmsBlack",
        PalmsColor = "palmsColor",
        PaperClips = "paperClips",
        Papyrus = "papyrus",
        PartyFavor = "partyFavor",
        PartyGlass = "partyGlass",
        Pencils = "pencils",
        People = "people",
        PeopleWaving = "peopleWaving",
        PeopleHats = "peopleHats",
        Poinsettias = "poinsettias",
        PostageStamp = "postageStamp",
        Pumpkin1 = "pumpkin1",
        PushPinNote2 = "pushPinNote2",
        PushPinNote1 = "pushPinNote1",
        Pyramids = "pyramids",
        PyramidsAbove = "pyramidsAbove",
        Quadrants = "quadrants",
        Rings = "rings",
        Safari = "safari",
        Sawtooth = "sawtooth",
        SawtoothGray = "sawtoothGray",
        ScaredCat = "scaredCat",
        Seattle = "seattle",
        ShadowedSquares = "shadowedSquares",
        SharksTeeth = "sharksTeeth",
        ShorebirdTracks = "shorebirdTracks",
        Skyrocket = "skyrocket",
        SnowflakeFancy = "snowflakeFancy",
        Snowflakes = "snowflakes",
        Sombrero = "sombrero",
        Southwest = "southwest",
        Stars = "stars",
        StarsTop = "starsTop",
        Stars3d = "stars3d",
        StarsBlack = "starsBlack",
        StarsShadowed = "starsShadowed",
        Sun = "sun",
        Swirligig = "swirligig",
        TornPaper = "tornPaper",
        TornPaperBlack = "tornPaperBlack",
        Trees = "trees",
        TriangleParty = "triangleParty",
        Triangles = "triangles",
        Tribal1 = "tribal1",
        Tribal2 = "tribal2",
        Tribal3 = "tribal3",
        Tribal4 = "tribal4",
        Tribal5 = "tribal5",
        Tribal6 = "tribal6",
        TwistedLines1 = "twistedLines1",
        TwistedLines2 = "twistedLines2",
        Vine = "vine",
        Waveline = "waveline",
        WeavingAngles = "weavingAngles",
        WeavingBraid = "weavingBraid",
        WeavingRibbon = "weavingRibbon",
        WeavingStrips = "weavingStrips",
        WhiteFlowers = "whiteFlowers",
        Woodwork = "woodwork",
        XIllusions = "xIllusions",
        ZanyTriangles = "zanyTriangles",
        ZigZag = "zigZag",
        ZigZagStitch = "zigZagStitch",
    }
}
