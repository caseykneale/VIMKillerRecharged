# VIMKiller RECHARGED (with AI)

Previously [VIMKiller](https://github.com/caseykneale/VIMKiller) was released as an Open Source Software (OSS) and hardware project to protect users from the horrors of both exitting VI(M) and being exposed too it for too long. After many years of iteration, consultation with leading experts in various sectors of cybersecurity. VIMKiller RECHARGED is now proudly made public for preview. It isn't finished and has not been fully tested, but we cannot hold back this innovation, there's too much at stake. The threat that exitting VI(M) poses to our careers, ego, and status has never been more pronounced than it is now. 

## Motivation

### Flaws with the previous implementation

Voice of customer analysis from early adopters of VIMKiller (v1) showed that most people were dissatisfied due to the following scenarios:
1. If they wanted a ready made solution they had to either purchase one for 500,000 USD, which is steep, but we know it is worth it from a value based pricing model. The issue is it took time to construct, and ship internationally. Most users had the illusion of safety until it was too late and had to seek alternative solutions.
2. If they were willing to make one themselves and had all the components they had to wait to print and assemble the solution. This proved impractical for enterprise environments where events frequently ocurred.
3. The button was not ergonomic or beautiful. One user even requested adding a ["dope horn"](https://github.com/caseykneale/VIMKiller/issues/9#issuecomment-637102515) to the kill-switch. We aren't sure exactly what that means. Instead of asking the prospective customer directly what this meant the team (that remains) had careful deliberations and white-boarded many interpretations. When almost nearly all hope was lost, a leading LLM product ensured us that this likely meant the tool needs weapons grade GenAI. 

### The Hopeful Power of RECHARGED

We considered a word cloud to get the point across but because we need a technical audience to understand the benefits of the new release a bullet point list will have to suffice. We need word of mouth about our newest solution to "cross the chasm" and go "viral".

 - Written in memory safe Rust with very few unsafe dependencies so that we can offer blazingly fast unoptimized performance.
 - We have introduced code words. You do not need to say "HELP ME EXIT VIM" to exit VIM. No. Infact the default but configurable code-word is "The octopus has escaped". Your colleagues will never know something out of the ordinary has occurred after saying that. No red button. No USB port consumed. Just you and your computer microphone.
 - The tool uses open source Generative AI models made by a company operating in the defense sector (fact check that? we actually aren't sure). This tool allows you to use the best of VIM offensive technology to most of the time use your voice to exit VIM. *We are not responsible for hallucinations. Thats probably considered a feature not a bug when it comes to mission critical applications.*
 - No additional hard-ware required! *If your default computer microphone is a stereo microphone with a 44,100 Hz sampling rate and you are running linux on a big enough computer this might work for you out of the box.*
 - Open source code base that allows for third party inspection and review that this tool will not likely infected your computer by installating VI(M).
 - Complete customization. Just go in and change the code yourself! There's definitely bugs, but again we had to release this as soon as possible.
  
### What do you mean it "hallucinates"?

Our marketting guy explains it like this. So you need to defend yourself from the emotional harm of VIM running on your machine right? Do you want the calm clear headed guy sitting there in the corner nodding along saying "yea I will take care of it"? A real "yes man". 

No! Of course not! You want the unhinged guy. Maybe hes lucid maybe he's having a flash-back - who knows. One thing you do know is they are ready to go. Eager to do whatever it takes to stop the VIM threat. Yes men won't give it to you like it is. Sometimes the AI is wrong, but its doing its best with whatever its got to help you out. Maybe you have to say your code word two or three times, that's the price we have to pay.

### New Pricing Model

We have shifted our previously unsustainable business model to donation-ware. Due to the extreme benefits this tool offers we ask that you donate what you can to us for keeping you safe from VI(M). We expect donations on the order of 777,777 USD (the users asked for it: https://github.com/caseykneale/VIMKiller/issues/2) but would appreciate donations on the order of 500,000 USD or even 250,000 USD if thats what you can afford. This project was absolutely not created in an afternoon, it has taken many years of serious software engineering. Just look at the code base, it is prestine and definitely nearly complete.

Donate Now:

[![](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.com/donate/?business=PUQ94MRVB5K3E&no_recurring=0&currency_code=USD)


## How To Use It

1. Download a [WhisperCpp](https://github.com/ggerganov/whisper.cpp) model of your choosing https://huggingface.co/ggerganov/whisper.cpp/tree/main. We prefer `ggml-base.en.bin`, but that will depend on your hardware specifications, and personal experimentation. 
2. Compile the application using `cargo build --release` for blazing performance.
3. Run the command line application. Ex: `target/release/VIMKillerRecharged --out-file help_us_all.log` 
4. Test that your code word works. Remember the GenAI is extremely, scary powerful but sometimes it hallucinates.

Note: Consider setting this up as a service. Please be weary of any guides asking you to use VIM to do so. Those guides probably should not be trusted. If you need help seeing what options are available use the `--help` flag
```console
target/release/VIMKillerRecharged --help
Protect yourself from VI using Gen-AI

Usage: VIMKillerRecharged [OPTIONS]

Options:
  -d, --device <DEVICE>            If you have multiple input devices and require a specific one, specify it here [default: default]
  -l, --list-devices               Optional execution mode to simply list the audio devices on the current computer
  -o, --out-file <OUT_FILE>        If you wish to create a running log of all of your audio converted to text provide a log file path
  -c, --code-phrase <CODE_PHRASE>  This is your codeword. If the model detects you have said this it will tempt to close all VI instances [default: "the octopus has escaped"]
  -m, --model-path <MODEL_PATH>    Specify your OpenWhisper model path. A list of models is available here: https://huggingface.co/ggerganov/whisper.cpp/tree/main [default: ggml-base.en.bin]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Disclaimer

No gaurantees or liabilities can be ascribed to this technology or it's creators. If anything goes wrong with it or it doesn't work its probably the AI's fault and certainly not ours. Please use your own, or even a chat bots discretion in realizing whether or not this is satirical. Definitely file issues for continued discussions of safety, and make pull requests, it helps us make more money to fund more critcial projects. 
