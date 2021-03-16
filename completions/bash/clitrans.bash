_clitrans() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            clitrans)
                cmd="clitrans"
                ;;
            
            completion)
                cmd+="__completion"
                ;;
            help)
                cmd+="__help"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        clitrans)
            opts=" -h -V -e -p -a  --help --version --engine --explanations --phonetics --phrases --audio  <QUERY>  completion help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --engine)
                    COMPREPLY=($(compgen -W "youdao bing" -- "${cur}"))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -W "youdao bing" -- "${cur}"))
                    return 0
                    ;;
                --explanations)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --phonetics)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --phrases)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --audio)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        clitrans__completion)
            opts=" -h -V  --help --version  <shell> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        clitrans__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _clitrans -o bashdefault -o default clitrans
