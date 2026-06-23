use crate::{ecs::EntityRef, entity_impl, ir::{KaliumBlockRef, func::KaliumFuncRef, stack::KaliumStackSlotRef, typ::KaliumType, value::{KaliumConstValue, KaliumGlobalValueRef, KaliumImm, KaliumStaticValueRef, KaliumValueRef}}, target::KaliumTargetSupport};

pub enum IntComparison {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    Inequal,
}

pub enum FloatComparison {
    Ordered,
    OrderedInequal,
    Unordered,
    UnorderedOrEqual,
    UnorderedOrLessThan,
    UnorderedOrLessThanOrEqual,
    UnorderedOrGreaterThan,
    UnorderedOrGreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    Inequal,
}

/// We actually use an enum because the assembler can simply check support later
/// and it is so so so so so much less clunky
pub enum KaliumInst {
    ConstValue(KaliumConstValue),

    Add(KaliumValueRef, KaliumValueRef, KaliumType),
    Sub(KaliumValueRef, KaliumValueRef, KaliumType),
    Mul(KaliumValueRef, KaliumValueRef, KaliumType),
    Div(KaliumValueRef, KaliumValueRef, KaliumType),
    Shl(KaliumValueRef, KaliumValueRef, KaliumType),
    Shr(KaliumValueRef, KaliumValueRef, KaliumType),

    IntNegate(KaliumValueRef, KaliumType),

    AddImm(KaliumValueRef, KaliumImm, KaliumType),
    SubImm(KaliumValueRef, KaliumImm, KaliumType),
    MulImm(KaliumValueRef, KaliumImm, KaliumType),
    DivImm(KaliumValueRef, KaliumImm, KaliumType),
    ShlImm(KaliumValueRef, KaliumImm, KaliumType),
    ShrImm(KaliumValueRef, KaliumImm, KaliumType),

    LogicalAnd(KaliumValueRef, KaliumValueRef),
    LogicalOr(KaliumValueRef, KaliumValueRef),
    LogicalNot(KaliumValueRef),

    BitwiseAnd(KaliumValueRef, KaliumValueRef, KaliumType),
    BitwiseOr(KaliumValueRef, KaliumValueRef, KaliumType),
    BitwiseXor(KaliumValueRef, KaliumValueRef, KaliumType),
    BitwiseNot(KaliumValueRef, KaliumValueRef, KaliumType),

    BitwiseAndImm(KaliumValueRef, KaliumImm, KaliumType),
    BitwiseOrImm(KaliumValueRef, KaliumImm, KaliumType),
    BitwiseXorImm(KaliumValueRef, KaliumImm, KaliumType),
    BitwiseNotImm(KaliumValueRef, KaliumImm, KaliumType),

    SignExtend(KaliumValueRef, KaliumType),
    ZeroExtend(KaliumValueRef, KaliumType),

    FuncAddr(KaliumFuncRef),
    StaticAddr(KaliumStaticValueRef),
    GlobalAddr(KaliumGlobalValueRef),

    BreakIf {
        then: KaliumBlockRef,
        r#else: KaliumBlockRef,
        arg: KaliumValueRef
    },

    Jump(KaliumBlockRef),

    Call(KaliumFuncRef, Box<[KaliumValueRef]>),
    CallIndirect(KaliumValueRef, Box<[KaliumValueRef]>),

    IntCompare(KaliumValueRef, KaliumValueRef, IntComparison),

    // The box reads "Vardanega".
    // The intricate lettering,
    // It oversells the nature of the contents.
    //
    // I would not ever have one,
    // Yet I find that one is gone,
    // And has been pressed
    // inbetween my lips,
    // And lit.
    //
    // I try to hold my breath
    // But I must breathe.
    //
    // I inhale.
    // I splutter.
    // And I move on.
    //
    // But later
    // I find myself back near the box.
    // Again
    // I find a cigarette missing,
    // And held at my mouth.
    //
    // In futility,
    // I hold my breath.
    // But I must inhale.
    // I cough.
    // And I ignore it.
    //
    // And now
    // The box has found me.
    // I inhale.
    // I exhale.
    // And I ignore it.
    //
    // And again.
    // I inhale.
    // I give it time.
    // I exhale.
    // And I ignore it.
    //
    // I know I should stop,
    // That it will only harm me,
    // But it pulls.
    // I cannot take my eyes off of the box.
    //
    // I inhale.
    // I exhale.
    // I move on.
    //
    // I inhale.
    // I exhale.
    // I ignore it.
    //
    // I inhale.
    // I exhale.
    // I dismiss it.
    //
    // But now
    // I am fed up.
    // But I cannot stop.
    //
    // I inhale
    // But ensure it doesn't enter my lungs.
    // I exhale.
    // And I ignore it.
    //
    // NOTE: this is a draft
    //
    // Also today at school I realised that it must be common knowledge in 8.5 that Mack is the direct cause for my switching classes. Also, I think Mrs. Ward may also now realise that something occurred that I do NOT want to discuss, as not only have I been receiving a hearty amunt of questioning in maths but someone (Charley Lyons, I think it was) brought up how it was because of Mack I moved classes and my discomfort was probably very hard to miss (I am almost sure it was, as even Mrs. Ward told them to stop asking)
    // Also, Mack was (probably) not at school today. I know this because when we had History 8.5 had Japanese in the room opposite, and he was not there
    // Also, during that, someone in 8.8 (I forget who) said I had moved because "Mack raped me", so clearly that rumour is solidifying too.
    // What did Albert Einstein (or at least someone) say insanity was... "Insanity is doing the same thing over and over again and expecting different results." Thats odd, because everyone KEEPS ASKING NO MATTER HOW MANY TIMES I DO NOT ANSWER
    // I like to think Mack was not at school as a result of these rumours, but perhaps I am too hopeful.
    // Actually, now I think about it, Mack must be getting the same, if not hopefully more, questions about why I left. I wonder what he has said...
    // And also, WHERE. DID. THE. RUMOUR. THAT. MACK. "TOUCHED" OR "RAPED" ME. COME. FROM!?!??!?!
    // Wait shit I forgot they actually questioned some witnesses
    // Maybe I should just confirm the rumours and see what happens
    // Like what would Letitia do if suddenly a whole year group, especially the one she is in charge of, had these rumours and had them confirmed, I bet she'd freak
    // Again though, nuclear option
    // I think
    // Tomorrow I must reclaim the narrative, so to speak
    // Not too much all at once though, like tomorrow I might just confirm that I moved because of Mack, or because "they couldn't move Mack".
    //
    //
    // THE 23RD
    //
    // Today was actually normal, and I only got asked about rumours once. The problem is that the rumour "Mack touched me" seems to have spread far and wide, from 8.5 to 8.8 and other classes. Basically, I think most of Year 8 has heard the rumour at this point. The other problem is who was asking (they will make sure to spread any knowledge far and wide).
    //
    // I did not see Mack, nor did I see 8.5 as a class, so he might have been here.
    //
    // The problem is that tomorrow I will spend at least a morning at CHS, and if my being there for a mere 30 minutes before school spread so far to the point that my absence was assumed to be my transferral, this ought to be even wilder.
    //
    // Also we may be pressing charges against Mack 😊
    // The people are going to have a field day with that one
    // Lol I am going to have to tell them
    // And then they too are going to have to make a mandatory report
    // Mack is fucked (but also hopefully going to receive the help he so desperately needs, and hopefully it won't happen again. As for the school that is yet to be seen)

    FloatCompare(KaliumValueRef, KaliumValueRef, FloatComparison),

    Load {
        ptr: KaliumValueRef,
        offset: KaliumValueRef,
        typ: KaliumType,
    },

    LoadImm {
        ptr: KaliumValueRef,
        offset: KaliumImm,
        typ: KaliumType,
    },

    Store {
        ptr: KaliumValueRef,
        offset: KaliumValueRef,
        val: KaliumValueRef,
        typ: KaliumType,
    },

    StoreImm {
        ptr: KaliumValueRef,
        offset: KaliumImm,
        val: KaliumValueRef,
        typ: KaliumType,
    },

    StackLoad {
        slot: KaliumStackSlotRef,
        offset: KaliumValueRef,
        typ: KaliumType,
    },

    StackLoadImm {
        slot: KaliumStackSlotRef,
        offset: KaliumImm,
        typ: KaliumType,
    },

    StackStore {
        slot: KaliumStackSlotRef,
        offset: KaliumValueRef,
        val: KaliumValueRef,
        typ: KaliumType,
    },

    StackStoreImm {
        slot: KaliumStackSlotRef,
        offset: KaliumImm,
        val: KaliumValueRef,
        typ: KaliumType,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KaliumInstRef(u32);
entity_impl!(KaliumInstRef : u32);


// Lol what if Mosaic was just one big macro
// Haha
// "What if"
//
// Oh also I am not like stealing those instrument samples
// No I found pianobook.co.uk like last night and have been downloading sample pack after sample pack
// Where else am I meant to find a Polaroid CL635 sample pack (yes it has all the sounds I need, which is none of them but also simultaneously all of them, including cartridge sounds, printing sounds, and flash sounds)?? Or A 3D-Printed Ocarina sample pack???? Or an Italian Typewriter sample pack??? Or the Bechstein C118 sample pack (the piano featured in my "cover")? Or the Seaside Transistor Organ & Drum Machine sample pack (the organ featured in my "cover")??? I CAN TASTE THE FREEDOM
// ...
// The freedom...
// Of still not feeling anything?
// S t i l l ?
// Why do I feel indifferent to the idea of him
// STILL
// And *what* was with the double take at the site of me????
// If I am not going to feel anything towards him might as well wonder what he feels towards me
// Also not only is it downright ridiculous they didn't even try to do anything in regards to preventing further abuse (regardless of victim), it is also just sad for everyone, really, because clearly he needs intense help, no well person can bring themselves to do that
// I still can't help but wonder what he was thinking during them
// If anyone hasn't been catastrophically failed in this situation it is the school still receiving possibly millions of dollars annually
//
// Actually
// What even brings one to do such a thing?
//
// Hey at least someone found me attra
// Actually no
// But there must have been some level of lust involved
//
// Wait that most recent one (which is still like 3 or 4 weeks ago at this point?) in Religion, where he tried to effectively hold hands with me (and resorted to stroking my fucking forearm) is especially weird. I do not like the implications that has as to what he thought our "relationship" was (abuser-victim is the correct answer)
//
// Spending a morning at CHS this Wednesday (I plan to stay at Trinity unless shit really hits the fan, because I will pretend it hasn't. I was forced into this)
// So far the only 2 days back at Trinity (it is Sunday... 01:52?!?!?!) have been fine outside of the fucking rumours
//
// Good thing the surface area for any social damage due to rumours is...
// Quite low already...
// yay...
// I love it when like one or two people can be absent from school and that day will suck
// Yes
// Wow
// Thats the best
//
// "If only I had some way out"
// As rebuttal: it is too late for that, I will have to endure
// See: CAHMS
//
// Okay fuck this it is 02:03 and I dont dare try to run the rest of the day on less than four hours of sleep
// So it is the morning after but I remember checking the time while trying to sleep and it was like 02:57 damn
// All in all assuming I was asleep at 03:00 I would have got about... like 4 - 4.5 hours of sleep?
// Wait where is my brown texta?? I use it *once* and now it disappears
// Oh nevermind found it
// (It won't be used for drawing or writing, but not for anything nefarious either, unless a=,”Vë¢| is nefarious, which I don't think it is, no matter how enjoyable it may be, but some people do)
// (Also, the texta will need to be kept separate after, and really should have been already, but oh well, unless I can very thoroughly cleanse and sanitise it)
// (And, this texta has been used for this purpose once before, and nearly a second already)
// (Also, Mack might be in my mind while using it, but I will try to have that not be the case, but that certainly failed last time)
// (Not NSSI, how could you even do that with a texta???)
// (Think what happened "later that same afternoon", on "that fateful day")
// (The marker will actually *need* to be cleansed and/or sanitised, with where its going)
// (ngl though the use of the brown texta in such a manner as described was quite nice)
// ...
// Well oops I was way underprepared this time around (oh well)
// That poor texta certainly needed sanitising