
edit:completion:arg-completer[clitrans] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'clitrans'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'clitrans'= {
            cand -e 'Translate engine'
            cand --engine 'Translate engine'
            cand --explanations 'How many explanations to display'
            cand --phonetics 'How many phonetics to display'
            cand -p 'How many web phrases to display'
            cand --phrases 'How many web phrases to display'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand completion 'Generate shell completion file'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'clitrans;completion'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'clitrans;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
