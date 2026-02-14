_pt-core() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="pt__core"
                ;;
            pt__core,agent)
                cmd="pt__core__agent"
                ;;
            pt__core,bundle)
                cmd="pt__core__bundle"
                ;;
            pt__core,check)
                cmd="pt__core__check"
                ;;
            pt__core,completions)
                cmd="pt__core__completions"
                ;;
            pt__core,config)
                cmd="pt__core__config"
                ;;
            pt__core,deep-scan)
                cmd="pt__core__deep__scan"
                ;;
            pt__core,diff)
                cmd="pt__core__diff"
                ;;
            pt__core,help)
                cmd="pt__core__help"
                ;;
            pt__core,learn)
                cmd="pt__core__learn"
                ;;
            pt__core,mcp)
                cmd="pt__core__mcp"
                ;;
            pt__core,query)
                cmd="pt__core__query"
                ;;
            pt__core,report)
                cmd="pt__core__report"
                ;;
            pt__core,robot)
                cmd="pt__core__agent"
                ;;
            pt__core,run)
                cmd="pt__core__run"
                ;;
            pt__core,scan)
                cmd="pt__core__scan"
                ;;
            pt__core,schema)
                cmd="pt__core__schema"
                ;;
            pt__core,shadow)
                cmd="pt__core__shadow"
                ;;
            pt__core,signature)
                cmd="pt__core__signature"
                ;;
            pt__core,telemetry)
                cmd="pt__core__telemetry"
                ;;
            pt__core,update)
                cmd="pt__core__update"
                ;;
            pt__core,version)
                cmd="pt__core__version"
                ;;
            pt__core__agent,apply)
                cmd="pt__core__agent__apply"
                ;;
            pt__core__agent,capabilities)
                cmd="pt__core__agent__capabilities"
                ;;
            pt__core__agent,diff)
                cmd="pt__core__agent__diff"
                ;;
            pt__core__agent,explain)
                cmd="pt__core__agent__explain"
                ;;
            pt__core__agent,export)
                cmd="pt__core__agent__export"
                ;;
            pt__core__agent,export-priors)
                cmd="pt__core__agent__export__priors"
                ;;
            pt__core__agent,fleet)
                cmd="pt__core__agent__fleet"
                ;;
            pt__core__agent,help)
                cmd="pt__core__agent__help"
                ;;
            pt__core__agent,import-priors)
                cmd="pt__core__agent__import__priors"
                ;;
            pt__core__agent,inbox)
                cmd="pt__core__agent__inbox"
                ;;
            pt__core__agent,init)
                cmd="pt__core__agent__init"
                ;;
            pt__core__agent,list-priors)
                cmd="pt__core__agent__list__priors"
                ;;
            pt__core__agent,plan)
                cmd="pt__core__agent__plan"
                ;;
            pt__core__agent,sessions)
                cmd="pt__core__agent__sessions"
                ;;
            pt__core__agent,snapshot)
                cmd="pt__core__agent__snapshot"
                ;;
            pt__core__agent,tail)
                cmd="pt__core__agent__tail"
                ;;
            pt__core__agent,verify)
                cmd="pt__core__agent__verify"
                ;;
            pt__core__agent,watch)
                cmd="pt__core__agent__watch"
                ;;
            pt__core__agent__fleet,apply)
                cmd="pt__core__agent__fleet__apply"
                ;;
            pt__core__agent__fleet,help)
                cmd="pt__core__agent__fleet__help"
                ;;
            pt__core__agent__fleet,plan)
                cmd="pt__core__agent__fleet__plan"
                ;;
            pt__core__agent__fleet,report)
                cmd="pt__core__agent__fleet__report"
                ;;
            pt__core__agent__fleet,status)
                cmd="pt__core__agent__fleet__status"
                ;;
            pt__core__agent__fleet,transfer)
                cmd="pt__core__agent__fleet__transfer"
                ;;
            pt__core__agent__fleet__help,apply)
                cmd="pt__core__agent__fleet__help__apply"
                ;;
            pt__core__agent__fleet__help,help)
                cmd="pt__core__agent__fleet__help__help"
                ;;
            pt__core__agent__fleet__help,plan)
                cmd="pt__core__agent__fleet__help__plan"
                ;;
            pt__core__agent__fleet__help,report)
                cmd="pt__core__agent__fleet__help__report"
                ;;
            pt__core__agent__fleet__help,status)
                cmd="pt__core__agent__fleet__help__status"
                ;;
            pt__core__agent__fleet__help,transfer)
                cmd="pt__core__agent__fleet__help__transfer"
                ;;
            pt__core__agent__fleet__help__transfer,diff)
                cmd="pt__core__agent__fleet__help__transfer__diff"
                ;;
            pt__core__agent__fleet__help__transfer,export)
                cmd="pt__core__agent__fleet__help__transfer__export"
                ;;
            pt__core__agent__fleet__help__transfer,import)
                cmd="pt__core__agent__fleet__help__transfer__import"
                ;;
            pt__core__agent__fleet__transfer,diff)
                cmd="pt__core__agent__fleet__transfer__diff"
                ;;
            pt__core__agent__fleet__transfer,export)
                cmd="pt__core__agent__fleet__transfer__export"
                ;;
            pt__core__agent__fleet__transfer,help)
                cmd="pt__core__agent__fleet__transfer__help"
                ;;
            pt__core__agent__fleet__transfer,import)
                cmd="pt__core__agent__fleet__transfer__import"
                ;;
            pt__core__agent__fleet__transfer__help,diff)
                cmd="pt__core__agent__fleet__transfer__help__diff"
                ;;
            pt__core__agent__fleet__transfer__help,export)
                cmd="pt__core__agent__fleet__transfer__help__export"
                ;;
            pt__core__agent__fleet__transfer__help,help)
                cmd="pt__core__agent__fleet__transfer__help__help"
                ;;
            pt__core__agent__fleet__transfer__help,import)
                cmd="pt__core__agent__fleet__transfer__help__import"
                ;;
            pt__core__agent__help,apply)
                cmd="pt__core__agent__help__apply"
                ;;
            pt__core__agent__help,capabilities)
                cmd="pt__core__agent__help__capabilities"
                ;;
            pt__core__agent__help,diff)
                cmd="pt__core__agent__help__diff"
                ;;
            pt__core__agent__help,explain)
                cmd="pt__core__agent__help__explain"
                ;;
            pt__core__agent__help,export)
                cmd="pt__core__agent__help__export"
                ;;
            pt__core__agent__help,export-priors)
                cmd="pt__core__agent__help__export__priors"
                ;;
            pt__core__agent__help,fleet)
                cmd="pt__core__agent__help__fleet"
                ;;
            pt__core__agent__help,help)
                cmd="pt__core__agent__help__help"
                ;;
            pt__core__agent__help,import-priors)
                cmd="pt__core__agent__help__import__priors"
                ;;
            pt__core__agent__help,inbox)
                cmd="pt__core__agent__help__inbox"
                ;;
            pt__core__agent__help,init)
                cmd="pt__core__agent__help__init"
                ;;
            pt__core__agent__help,list-priors)
                cmd="pt__core__agent__help__list__priors"
                ;;
            pt__core__agent__help,plan)
                cmd="pt__core__agent__help__plan"
                ;;
            pt__core__agent__help,sessions)
                cmd="pt__core__agent__help__sessions"
                ;;
            pt__core__agent__help,snapshot)
                cmd="pt__core__agent__help__snapshot"
                ;;
            pt__core__agent__help,tail)
                cmd="pt__core__agent__help__tail"
                ;;
            pt__core__agent__help,verify)
                cmd="pt__core__agent__help__verify"
                ;;
            pt__core__agent__help,watch)
                cmd="pt__core__agent__help__watch"
                ;;
            pt__core__agent__help__fleet,apply)
                cmd="pt__core__agent__help__fleet__apply"
                ;;
            pt__core__agent__help__fleet,plan)
                cmd="pt__core__agent__help__fleet__plan"
                ;;
            pt__core__agent__help__fleet,report)
                cmd="pt__core__agent__help__fleet__report"
                ;;
            pt__core__agent__help__fleet,status)
                cmd="pt__core__agent__help__fleet__status"
                ;;
            pt__core__agent__help__fleet,transfer)
                cmd="pt__core__agent__help__fleet__transfer"
                ;;
            pt__core__agent__help__fleet__transfer,diff)
                cmd="pt__core__agent__help__fleet__transfer__diff"
                ;;
            pt__core__agent__help__fleet__transfer,export)
                cmd="pt__core__agent__help__fleet__transfer__export"
                ;;
            pt__core__agent__help__fleet__transfer,import)
                cmd="pt__core__agent__help__fleet__transfer__import"
                ;;
            pt__core__bundle,create)
                cmd="pt__core__bundle__create"
                ;;
            pt__core__bundle,extract)
                cmd="pt__core__bundle__extract"
                ;;
            pt__core__bundle,help)
                cmd="pt__core__bundle__help"
                ;;
            pt__core__bundle,inspect)
                cmd="pt__core__bundle__inspect"
                ;;
            pt__core__bundle__help,create)
                cmd="pt__core__bundle__help__create"
                ;;
            pt__core__bundle__help,extract)
                cmd="pt__core__bundle__help__extract"
                ;;
            pt__core__bundle__help,help)
                cmd="pt__core__bundle__help__help"
                ;;
            pt__core__bundle__help,inspect)
                cmd="pt__core__bundle__help__inspect"
                ;;
            pt__core__config,diff-preset)
                cmd="pt__core__config__diff__preset"
                ;;
            pt__core__config,export-preset)
                cmd="pt__core__config__export__preset"
                ;;
            pt__core__config,help)
                cmd="pt__core__config__help"
                ;;
            pt__core__config,list-presets)
                cmd="pt__core__config__list__presets"
                ;;
            pt__core__config,schema)
                cmd="pt__core__config__schema"
                ;;
            pt__core__config,show)
                cmd="pt__core__config__show"
                ;;
            pt__core__config,show-preset)
                cmd="pt__core__config__show__preset"
                ;;
            pt__core__config,validate)
                cmd="pt__core__config__validate"
                ;;
            pt__core__config__help,diff-preset)
                cmd="pt__core__config__help__diff__preset"
                ;;
            pt__core__config__help,export-preset)
                cmd="pt__core__config__help__export__preset"
                ;;
            pt__core__config__help,help)
                cmd="pt__core__config__help__help"
                ;;
            pt__core__config__help,list-presets)
                cmd="pt__core__config__help__list__presets"
                ;;
            pt__core__config__help,schema)
                cmd="pt__core__config__help__schema"
                ;;
            pt__core__config__help,show)
                cmd="pt__core__config__help__show"
                ;;
            pt__core__config__help,show-preset)
                cmd="pt__core__config__help__show__preset"
                ;;
            pt__core__config__help,validate)
                cmd="pt__core__config__help__validate"
                ;;
            pt__core__help,agent)
                cmd="pt__core__help__agent"
                ;;
            pt__core__help,bundle)
                cmd="pt__core__help__bundle"
                ;;
            pt__core__help,check)
                cmd="pt__core__help__check"
                ;;
            pt__core__help,completions)
                cmd="pt__core__help__completions"
                ;;
            pt__core__help,config)
                cmd="pt__core__help__config"
                ;;
            pt__core__help,deep-scan)
                cmd="pt__core__help__deep__scan"
                ;;
            pt__core__help,diff)
                cmd="pt__core__help__diff"
                ;;
            pt__core__help,help)
                cmd="pt__core__help__help"
                ;;
            pt__core__help,learn)
                cmd="pt__core__help__learn"
                ;;
            pt__core__help,mcp)
                cmd="pt__core__help__mcp"
                ;;
            pt__core__help,query)
                cmd="pt__core__help__query"
                ;;
            pt__core__help,report)
                cmd="pt__core__help__report"
                ;;
            pt__core__help,run)
                cmd="pt__core__help__run"
                ;;
            pt__core__help,scan)
                cmd="pt__core__help__scan"
                ;;
            pt__core__help,schema)
                cmd="pt__core__help__schema"
                ;;
            pt__core__help,shadow)
                cmd="pt__core__help__shadow"
                ;;
            pt__core__help,signature)
                cmd="pt__core__help__signature"
                ;;
            pt__core__help,telemetry)
                cmd="pt__core__help__telemetry"
                ;;
            pt__core__help,update)
                cmd="pt__core__help__update"
                ;;
            pt__core__help,version)
                cmd="pt__core__help__version"
                ;;
            pt__core__help__agent,apply)
                cmd="pt__core__help__agent__apply"
                ;;
            pt__core__help__agent,capabilities)
                cmd="pt__core__help__agent__capabilities"
                ;;
            pt__core__help__agent,diff)
                cmd="pt__core__help__agent__diff"
                ;;
            pt__core__help__agent,explain)
                cmd="pt__core__help__agent__explain"
                ;;
            pt__core__help__agent,export)
                cmd="pt__core__help__agent__export"
                ;;
            pt__core__help__agent,export-priors)
                cmd="pt__core__help__agent__export__priors"
                ;;
            pt__core__help__agent,fleet)
                cmd="pt__core__help__agent__fleet"
                ;;
            pt__core__help__agent,import-priors)
                cmd="pt__core__help__agent__import__priors"
                ;;
            pt__core__help__agent,inbox)
                cmd="pt__core__help__agent__inbox"
                ;;
            pt__core__help__agent,init)
                cmd="pt__core__help__agent__init"
                ;;
            pt__core__help__agent,list-priors)
                cmd="pt__core__help__agent__list__priors"
                ;;
            pt__core__help__agent,plan)
                cmd="pt__core__help__agent__plan"
                ;;
            pt__core__help__agent,sessions)
                cmd="pt__core__help__agent__sessions"
                ;;
            pt__core__help__agent,snapshot)
                cmd="pt__core__help__agent__snapshot"
                ;;
            pt__core__help__agent,tail)
                cmd="pt__core__help__agent__tail"
                ;;
            pt__core__help__agent,verify)
                cmd="pt__core__help__agent__verify"
                ;;
            pt__core__help__agent,watch)
                cmd="pt__core__help__agent__watch"
                ;;
            pt__core__help__agent__fleet,apply)
                cmd="pt__core__help__agent__fleet__apply"
                ;;
            pt__core__help__agent__fleet,plan)
                cmd="pt__core__help__agent__fleet__plan"
                ;;
            pt__core__help__agent__fleet,report)
                cmd="pt__core__help__agent__fleet__report"
                ;;
            pt__core__help__agent__fleet,status)
                cmd="pt__core__help__agent__fleet__status"
                ;;
            pt__core__help__agent__fleet,transfer)
                cmd="pt__core__help__agent__fleet__transfer"
                ;;
            pt__core__help__agent__fleet__transfer,diff)
                cmd="pt__core__help__agent__fleet__transfer__diff"
                ;;
            pt__core__help__agent__fleet__transfer,export)
                cmd="pt__core__help__agent__fleet__transfer__export"
                ;;
            pt__core__help__agent__fleet__transfer,import)
                cmd="pt__core__help__agent__fleet__transfer__import"
                ;;
            pt__core__help__bundle,create)
                cmd="pt__core__help__bundle__create"
                ;;
            pt__core__help__bundle,extract)
                cmd="pt__core__help__bundle__extract"
                ;;
            pt__core__help__bundle,inspect)
                cmd="pt__core__help__bundle__inspect"
                ;;
            pt__core__help__config,diff-preset)
                cmd="pt__core__help__config__diff__preset"
                ;;
            pt__core__help__config,export-preset)
                cmd="pt__core__help__config__export__preset"
                ;;
            pt__core__help__config,list-presets)
                cmd="pt__core__help__config__list__presets"
                ;;
            pt__core__help__config,schema)
                cmd="pt__core__help__config__schema"
                ;;
            pt__core__help__config,show)
                cmd="pt__core__help__config__show"
                ;;
            pt__core__help__config,show-preset)
                cmd="pt__core__help__config__show__preset"
                ;;
            pt__core__help__config,validate)
                cmd="pt__core__help__config__validate"
                ;;
            pt__core__help__learn,complete)
                cmd="pt__core__help__learn__complete"
                ;;
            pt__core__help__learn,list)
                cmd="pt__core__help__learn__list"
                ;;
            pt__core__help__learn,reset)
                cmd="pt__core__help__learn__reset"
                ;;
            pt__core__help__learn,show)
                cmd="pt__core__help__learn__show"
                ;;
            pt__core__help__learn,verify)
                cmd="pt__core__help__learn__verify"
                ;;
            pt__core__help__query,actions)
                cmd="pt__core__help__query__actions"
                ;;
            pt__core__help__query,sessions)
                cmd="pt__core__help__query__sessions"
                ;;
            pt__core__help__query,telemetry)
                cmd="pt__core__help__query__telemetry"
                ;;
            pt__core__help__shadow,export)
                cmd="pt__core__help__shadow__export"
                ;;
            pt__core__help__shadow,report)
                cmd="pt__core__help__shadow__report"
                ;;
            pt__core__help__shadow,run)
                cmd="pt__core__help__shadow__run"
                ;;
            pt__core__help__shadow,start)
                cmd="pt__core__help__shadow__start"
                ;;
            pt__core__help__shadow,status)
                cmd="pt__core__help__shadow__status"
                ;;
            pt__core__help__shadow,stop)
                cmd="pt__core__help__shadow__stop"
                ;;
            pt__core__help__signature,add)
                cmd="pt__core__help__signature__add"
                ;;
            pt__core__help__signature,disable)
                cmd="pt__core__help__signature__disable"
                ;;
            pt__core__help__signature,enable)
                cmd="pt__core__help__signature__enable"
                ;;
            pt__core__help__signature,export)
                cmd="pt__core__help__signature__export"
                ;;
            pt__core__help__signature,import)
                cmd="pt__core__help__signature__import"
                ;;
            pt__core__help__signature,list)
                cmd="pt__core__help__signature__list"
                ;;
            pt__core__help__signature,remove)
                cmd="pt__core__help__signature__remove"
                ;;
            pt__core__help__signature,show)
                cmd="pt__core__help__signature__show"
                ;;
            pt__core__help__signature,stats)
                cmd="pt__core__help__signature__stats"
                ;;
            pt__core__help__signature,test)
                cmd="pt__core__help__signature__test"
                ;;
            pt__core__help__signature,validate)
                cmd="pt__core__help__signature__validate"
                ;;
            pt__core__help__telemetry,export)
                cmd="pt__core__help__telemetry__export"
                ;;
            pt__core__help__telemetry,prune)
                cmd="pt__core__help__telemetry__prune"
                ;;
            pt__core__help__telemetry,redact)
                cmd="pt__core__help__telemetry__redact"
                ;;
            pt__core__help__telemetry,status)
                cmd="pt__core__help__telemetry__status"
                ;;
            pt__core__help__update,list-backups)
                cmd="pt__core__help__update__list__backups"
                ;;
            pt__core__help__update,prune-backups)
                cmd="pt__core__help__update__prune__backups"
                ;;
            pt__core__help__update,rollback)
                cmd="pt__core__help__update__rollback"
                ;;
            pt__core__help__update,show-backup)
                cmd="pt__core__help__update__show__backup"
                ;;
            pt__core__help__update,verify-backup)
                cmd="pt__core__help__update__verify__backup"
                ;;
            pt__core__learn,complete)
                cmd="pt__core__learn__complete"
                ;;
            pt__core__learn,help)
                cmd="pt__core__learn__help"
                ;;
            pt__core__learn,list)
                cmd="pt__core__learn__list"
                ;;
            pt__core__learn,reset)
                cmd="pt__core__learn__reset"
                ;;
            pt__core__learn,show)
                cmd="pt__core__learn__show"
                ;;
            pt__core__learn,verify)
                cmd="pt__core__learn__verify"
                ;;
            pt__core__learn__help,complete)
                cmd="pt__core__learn__help__complete"
                ;;
            pt__core__learn__help,help)
                cmd="pt__core__learn__help__help"
                ;;
            pt__core__learn__help,list)
                cmd="pt__core__learn__help__list"
                ;;
            pt__core__learn__help,reset)
                cmd="pt__core__learn__help__reset"
                ;;
            pt__core__learn__help,show)
                cmd="pt__core__learn__help__show"
                ;;
            pt__core__learn__help,verify)
                cmd="pt__core__learn__help__verify"
                ;;
            pt__core__query,actions)
                cmd="pt__core__query__actions"
                ;;
            pt__core__query,help)
                cmd="pt__core__query__help"
                ;;
            pt__core__query,sessions)
                cmd="pt__core__query__sessions"
                ;;
            pt__core__query,telemetry)
                cmd="pt__core__query__telemetry"
                ;;
            pt__core__query__help,actions)
                cmd="pt__core__query__help__actions"
                ;;
            pt__core__query__help,help)
                cmd="pt__core__query__help__help"
                ;;
            pt__core__query__help,sessions)
                cmd="pt__core__query__help__sessions"
                ;;
            pt__core__query__help,telemetry)
                cmd="pt__core__query__help__telemetry"
                ;;
            pt__core__shadow,export)
                cmd="pt__core__shadow__export"
                ;;
            pt__core__shadow,help)
                cmd="pt__core__shadow__help"
                ;;
            pt__core__shadow,report)
                cmd="pt__core__shadow__report"
                ;;
            pt__core__shadow,run)
                cmd="pt__core__shadow__run"
                ;;
            pt__core__shadow,start)
                cmd="pt__core__shadow__start"
                ;;
            pt__core__shadow,status)
                cmd="pt__core__shadow__status"
                ;;
            pt__core__shadow,stop)
                cmd="pt__core__shadow__stop"
                ;;
            pt__core__shadow__help,export)
                cmd="pt__core__shadow__help__export"
                ;;
            pt__core__shadow__help,help)
                cmd="pt__core__shadow__help__help"
                ;;
            pt__core__shadow__help,report)
                cmd="pt__core__shadow__help__report"
                ;;
            pt__core__shadow__help,run)
                cmd="pt__core__shadow__help__run"
                ;;
            pt__core__shadow__help,start)
                cmd="pt__core__shadow__help__start"
                ;;
            pt__core__shadow__help,status)
                cmd="pt__core__shadow__help__status"
                ;;
            pt__core__shadow__help,stop)
                cmd="pt__core__shadow__help__stop"
                ;;
            pt__core__signature,add)
                cmd="pt__core__signature__add"
                ;;
            pt__core__signature,disable)
                cmd="pt__core__signature__disable"
                ;;
            pt__core__signature,enable)
                cmd="pt__core__signature__enable"
                ;;
            pt__core__signature,export)
                cmd="pt__core__signature__export"
                ;;
            pt__core__signature,help)
                cmd="pt__core__signature__help"
                ;;
            pt__core__signature,import)
                cmd="pt__core__signature__import"
                ;;
            pt__core__signature,list)
                cmd="pt__core__signature__list"
                ;;
            pt__core__signature,remove)
                cmd="pt__core__signature__remove"
                ;;
            pt__core__signature,show)
                cmd="pt__core__signature__show"
                ;;
            pt__core__signature,stats)
                cmd="pt__core__signature__stats"
                ;;
            pt__core__signature,test)
                cmd="pt__core__signature__test"
                ;;
            pt__core__signature,validate)
                cmd="pt__core__signature__validate"
                ;;
            pt__core__signature__help,add)
                cmd="pt__core__signature__help__add"
                ;;
            pt__core__signature__help,disable)
                cmd="pt__core__signature__help__disable"
                ;;
            pt__core__signature__help,enable)
                cmd="pt__core__signature__help__enable"
                ;;
            pt__core__signature__help,export)
                cmd="pt__core__signature__help__export"
                ;;
            pt__core__signature__help,help)
                cmd="pt__core__signature__help__help"
                ;;
            pt__core__signature__help,import)
                cmd="pt__core__signature__help__import"
                ;;
            pt__core__signature__help,list)
                cmd="pt__core__signature__help__list"
                ;;
            pt__core__signature__help,remove)
                cmd="pt__core__signature__help__remove"
                ;;
            pt__core__signature__help,show)
                cmd="pt__core__signature__help__show"
                ;;
            pt__core__signature__help,stats)
                cmd="pt__core__signature__help__stats"
                ;;
            pt__core__signature__help,test)
                cmd="pt__core__signature__help__test"
                ;;
            pt__core__signature__help,validate)
                cmd="pt__core__signature__help__validate"
                ;;
            pt__core__telemetry,export)
                cmd="pt__core__telemetry__export"
                ;;
            pt__core__telemetry,help)
                cmd="pt__core__telemetry__help"
                ;;
            pt__core__telemetry,prune)
                cmd="pt__core__telemetry__prune"
                ;;
            pt__core__telemetry,redact)
                cmd="pt__core__telemetry__redact"
                ;;
            pt__core__telemetry,status)
                cmd="pt__core__telemetry__status"
                ;;
            pt__core__telemetry__help,export)
                cmd="pt__core__telemetry__help__export"
                ;;
            pt__core__telemetry__help,help)
                cmd="pt__core__telemetry__help__help"
                ;;
            pt__core__telemetry__help,prune)
                cmd="pt__core__telemetry__help__prune"
                ;;
            pt__core__telemetry__help,redact)
                cmd="pt__core__telemetry__help__redact"
                ;;
            pt__core__telemetry__help,status)
                cmd="pt__core__telemetry__help__status"
                ;;
            pt__core__update,help)
                cmd="pt__core__update__help"
                ;;
            pt__core__update,list-backups)
                cmd="pt__core__update__list__backups"
                ;;
            pt__core__update,prune-backups)
                cmd="pt__core__update__prune__backups"
                ;;
            pt__core__update,rollback)
                cmd="pt__core__update__rollback"
                ;;
            pt__core__update,show-backup)
                cmd="pt__core__update__show__backup"
                ;;
            pt__core__update,verify-backup)
                cmd="pt__core__update__verify__backup"
                ;;
            pt__core__update__help,help)
                cmd="pt__core__update__help__help"
                ;;
            pt__core__update__help,list-backups)
                cmd="pt__core__update__help__list__backups"
                ;;
            pt__core__update__help,prune-backups)
                cmd="pt__core__update__help__prune__backups"
                ;;
            pt__core__update__help,rollback)
                cmd="pt__core__update__help__rollback"
                ;;
            pt__core__update__help,show-backup)
                cmd="pt__core__update__help__show__backup"
                ;;
            pt__core__update__help,verify-backup)
                cmd="pt__core__update__help__verify__backup"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        pt__core)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version run scan deep-scan diff query bundle report check learn agent robot config telemetry shadow signature schema update mcp completions version help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version plan explain apply verify diff snapshot capabilities sessions list-priors inbox tail watch export-priors import-priors init export fleet help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__apply)
            opts="-f -v -q -h -V --session --pids --targets --yes --recommended --min-age --min-posterior --max-blast-radius --max-total-blast-radius --max-kills --require-known-signature --only-categories --exclude-categories --abort-on-unknown --resume --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pids)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --targets)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-age)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-posterior)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-blast-radius)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-total-blast-radius)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-kills)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --only-categories)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --exclude-categories)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__capabilities)
            opts="-f -v -q -h -V --check-action --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --check-action)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__diff)
            opts="-f -v -q -h -V --base --compare --focus --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --base)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --compare)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --focus)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__explain)
            opts="-f -v -q -h -V --session --pids --target --include --galaxy-brain --show-dependencies --show-blast-radius --show-history --what-if --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pids)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --target)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --include)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__export)
            opts="-o -f -v -q -h -V --session --out --profile --include-telemetry --include-dumps --encrypt --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__export__priors)
            opts="-o -f -v -q -h -V --out --host-profile --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version plan apply report status transfer help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__apply)
            opts="-f -v -q -h -V --fleet-session --parallel --timeout --continue-on-error --capabilities --config --format --verbose --quiet --no-color --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --fleet-session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --parallel)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__help)
            opts="plan apply report status transfer help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__fleet__help__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__help__plan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__help__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__help__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__help__transfer)
            opts="export import diff"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__help__transfer__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__help__transfer__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__help__transfer__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__plan)
            opts="-f -v -q -h -V --hosts --inventory --discovery-config --parallel --timeout --continue-on-error --host-profile --label --max-fdr --capabilities --config --format --verbose --quiet --no-color --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --hosts)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --inventory)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --discovery-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --parallel)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --label)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-fdr)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__report)
            opts="-f -v -q -h -V --fleet-session --out --profile --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --fleet-session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__status)
            opts="-f -v -q -h -V --fleet-session --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --fleet-session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__transfer)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version export import diff help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__transfer__diff)
            opts="-f -v -q -h -V --from --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --from)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__transfer__export)
            opts="-o -f -v -q -h -V --out --host-profile --include-signatures --include-priors --export-profile --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --export-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__fleet__transfer__help)
            opts="export import diff help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__fleet__transfer__help__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__transfer__help__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__transfer__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__transfer__help__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__fleet__transfer__import)
            opts="-f -v -q -h -V --from --merge-strategy --dry-run --no-backup --passphrase --normalize-baseline --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --from)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --merge-strategy)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__help)
            opts="plan explain apply verify diff snapshot capabilities sessions list-priors inbox tail watch export-priors import-priors init export fleet help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__agent__help__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__explain)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__export__priors)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__fleet)
            opts="plan apply report status transfer"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__fleet__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__help__fleet__plan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__help__fleet__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__help__fleet__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__help__fleet__transfer)
            opts="export import diff"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__agent__help__fleet__transfer__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__help__fleet__transfer__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__help__fleet__transfer__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__agent__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__import__priors)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__inbox)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__list__priors)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__plan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__sessions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__snapshot)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__tail)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__verify)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__help__watch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__agent__import__priors)
            opts="-i -f -v -q -h -V --from --merge --replace --host-profile --dry-run --no-backup --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --from)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host-profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__inbox)
            opts="-f -v -q -h -V --ack --clear --clear-all --unread --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --ack)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__init)
            opts="-f -v -q -h -V --yes --dry-run --agent --skip-backup --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --agent)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__list__priors)
            opts="-f -v -q -h -V --class --extended --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --class)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__plan)
            opts="-f -v -q -h -V --session --label --max-candidates --threshold --min-posterior --only --yes --include-kernel-threads --deep --min-age --sample-size --include-predictions --prediction-fields --since --since-time --goal --minimal --pretty --brief --narrative --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --label)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-candidates)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-posterior)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --only)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-age)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sample-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --prediction-fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --since)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --since-time)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --goal)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__sessions)
            opts="-f -v -q -h -V --session --detail --limit --state --cleanup --older-than --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --state)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --older-than)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__snapshot)
            opts="-f -v -q -h -V --label --top --include-env --include-network --minimal --pretty --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --label)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --top)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__tail)
            opts="-f -v -q -h -V --session --follow --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__verify)
            opts="-f -v -q -h -V --session --wait --check-respawn --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wait)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__agent__watch)
            opts="-f -v -q -h -V --notify-exec --notify-cmd --notify-arg --threshold --interval --min-age --once --goal-memory-available-gb --goal-load-max --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --notify-exec)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notify-cmd)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notify-arg)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-age)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --goal-memory-available-gb)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --goal-load-max)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__bundle)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version create inspect extract help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__bundle__create)
            opts="-o -f -v -q -h -V --session --output --profile --include-telemetry --include-dumps --encrypt --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --profile)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__bundle__extract)
            opts="-o -f -v -q -h -V --output --verify --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__bundle__help)
            opts="create inspect extract help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__bundle__help__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__bundle__help__extract)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__bundle__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__bundle__help__inspect)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__bundle__inspect)
            opts="-f -v -q -h -V --verify --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <PATH>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__check)
            opts="-f -v -q -h -V --priors --policy --check-capabilities --all --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__completions)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version bash elvish fish powershell zsh"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version show schema validate list-presets show-preset diff-preset export-preset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__diff__preset)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <PRESET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__export__preset)
            opts="-o -f -v -q -h -V --output --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <PRESET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__help)
            opts="show schema validate list-presets show-preset diff-preset export-preset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__config__help__diff__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__export__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__list__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__schema)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__show__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__help__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__config__list__presets)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__schema)
            opts="-f -v -q -h -V --file --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__show)
            opts="-f -v -q -h -V --file --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__show__preset)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <PRESET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__config__validate)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version [PATH]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__deep__scan)
            opts="-f -v -q -h -V --pids --budget --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --pids)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --budget)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__diff)
            opts="-f -v -q -h -V --baseline --last --changed-only --category --min-score-delta --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version [BASE] [COMPARE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --category)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-score-delta)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__help)
            opts="run scan deep-scan diff query bundle report check learn agent config telemetry shadow signature schema update mcp completions version help"
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
        pt__core__help__agent)
            opts="plan explain apply verify diff snapshot capabilities sessions list-priors inbox tail watch export-priors import-priors init export fleet"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__agent__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__capabilities)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__explain)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__export__priors)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__fleet)
            opts="plan apply report status transfer"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__fleet__apply)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__help__agent__fleet__plan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__help__agent__fleet__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__help__agent__fleet__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__help__agent__fleet__transfer)
            opts="export import diff"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 5 ]] ; then
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
        pt__core__help__agent__fleet__transfer__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__help__agent__fleet__transfer__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__help__agent__fleet__transfer__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 6 ]] ; then
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
        pt__core__help__agent__import__priors)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__inbox)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__list__priors)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__plan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__sessions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__snapshot)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__tail)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__verify)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__agent__watch)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__bundle)
            opts="create inspect extract"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__bundle__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__bundle__extract)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__bundle__inspect)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__completions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__config)
            opts="show schema validate list-presets show-preset diff-preset export-preset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__config__diff__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__config__export__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__config__list__presets)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__config__schema)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__config__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__config__show__preset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__config__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__deep__scan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__diff)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__learn)
            opts="list show verify complete reset"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__learn__complete)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__learn__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__learn__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__learn__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__learn__verify)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__mcp)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__query)
            opts="sessions actions telemetry"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__query__actions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__query__sessions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__query__telemetry)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__scan)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__schema)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__shadow)
            opts="start run stop status export report"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__shadow__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__shadow__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__shadow__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__shadow__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__shadow__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__shadow__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature)
            opts="list show add remove test validate export disable enable import stats"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__signature__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__signature__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__telemetry)
            opts="status export prune redact"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__telemetry__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__telemetry__prune)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__telemetry__redact)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__telemetry__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__update)
            opts="rollback list-backups show-backup verify-backup prune-backups"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__help__update__list__backups)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__update__prune__backups)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__update__rollback)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__update__show__backup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__update__verify__backup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__help__version)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__learn)
            opts="-f -v -q -h -V --verify-budget-ms --total-budget-ms --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version list show verify complete reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --verify-budget-ms)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --total-budget-ms)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__learn__complete)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <TOPIC>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__learn__help)
            opts="list show verify complete reset help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__learn__help__complete)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__learn__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__learn__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__learn__help__reset)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__learn__help__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__learn__help__verify)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__learn__list)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__learn__reset)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__learn__show)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <TOPIC>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__learn__verify)
            opts="-f -v -q -h -V --all --mark-complete --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version [TOPIC]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__mcp)
            opts="-f -v -q -h -V --transport --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --transport)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__query)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version [QUERY] sessions actions telemetry help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__query__actions)
            opts="-f -v -q -h -V --session --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__query__help)
            opts="sessions actions telemetry help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__query__help__actions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__query__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__query__help__sessions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__query__help__telemetry)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__query__sessions)
            opts="-f -v -q -h -V --limit --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__query__telemetry)
            opts="-f -v -q -h -V --range --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --range)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__report)
            opts="-o -f -v -q -h -V --session --output --include-ledger --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --session)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__run)
            opts="-f -v -q -h -V --deep --inline --signatures --community-signatures --min-age --goal --theme --high-contrast --reduce-motion --accessible --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --signatures)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-age)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --goal)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --theme)
                    COMPREPLY=($(compgen -W "dark light high-contrast no-color" -- "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__scan)
            opts="-f -v -q -h -V --deep --samples --interval --include-kernel-threads --goal --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --samples)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --goal)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__schema)
            opts="-l -a -f -v -q -h -V --list --all --compact --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --max-tokens --estimate-tokens --help --version [TYPE]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version start run stop status export report help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow__export)
            opts="-o -f -v -q -h -V --output --export-format --limit --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --export-format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow__help)
            opts="start run stop status export report help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__shadow__help__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__help__report)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__help__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__help__start)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__help__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__help__stop)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__shadow__report)
            opts="-o -f -v -q -h -V --output --threshold --limit --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow__run)
            opts="-f -v -q -h -V --interval --deep-interval --iterations --background --max-candidates --min-posterior --only --include-kernel-threads --deep --min-age --sample-size --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --deep-interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --iterations)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-candidates)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-posterior)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --only)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-age)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sample-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow__start)
            opts="-f -v -q -h -V --interval --deep-interval --iterations --background --max-candidates --min-posterior --only --include-kernel-threads --deep --min-age --sample-size --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --deep-interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --iterations)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-candidates)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-posterior)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --only)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --min-age)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sample-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow__status)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__shadow__stop)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version list show add remove test validate export disable enable import stats help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__add)
            opts="-f -v -q -h -V --category --pattern --arg-pattern --env-var --confidence --notes --priority --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --category)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pattern)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --arg-pattern)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --env-var)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --confidence)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --priority)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__disable)
            opts="-f -v -q -h -V --reason --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --reason)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__enable)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__export)
            opts="-f -v -q -h -V --user-only --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <OUTPUT>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__help)
            opts="list show add remove test validate export disable enable import stats help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__signature__help__add)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__disable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__enable)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__import)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__remove)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__show)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__stats)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__help__validate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__signature__import)
            opts="-f -v -q -h -V --dry-run --passphrase --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --standalone --fields --compact --max-tokens --estimate-tokens --help --version <INPUT>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --passphrase)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__list)
            opts="-f -v -q -h -V --user-only --builtin-only --category --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --category)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__remove)
            opts="-f -v -q -h -V --force --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__show)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__stats)
            opts="-f -v -q -h -V --min-matches --sort --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --min-matches)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sort)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__test)
            opts="-f -v -q -h -V --cmdline --all --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <PROCESS_NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cmdline)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__signature__validate)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__telemetry)
            opts="-f -v -q -h -V --telemetry-dir --retention-config --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version status export prune redact help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --telemetry-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --retention-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__telemetry__export)
            opts="-o -v -q -h -V --output --format --telemetry-dir --retention-config --capabilities --config --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --telemetry-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --retention-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__telemetry__help)
            opts="status export prune redact help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__telemetry__help__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__telemetry__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__telemetry__help__prune)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__telemetry__help__redact)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__telemetry__help__status)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__telemetry__prune)
            opts="-f -v -q -h -V --keep --dry-run --keep-everything --telemetry-dir --retention-config --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --keep)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --telemetry-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --retention-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__telemetry__redact)
            opts="-f -v -q -h -V --all --telemetry-dir --retention-config --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --telemetry-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --retention-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__telemetry__status)
            opts="-f -v -q -h -V --telemetry-dir --retention-config --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --telemetry-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --retention-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__update)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version rollback list-backups show-backup verify-backup prune-backups help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__update__help)
            opts="rollback list-backups show-backup verify-backup prune-backups help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
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
        pt__core__update__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__update__help__list__backups)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__update__help__prune__backups)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__update__help__rollback)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__update__help__show__backup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__update__help__verify__backup)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
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
        pt__core__update__list__backups)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__update__prune__backups)
            opts="-f -v -q -h -V --keep --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --keep)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__update__rollback)
            opts="-f -v -q -h -V --force --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version [TARGET]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__update__show__backup)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version <TARGET>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__update__verify__backup)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version [TARGET]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
        pt__core__version)
            opts="-f -v -q -h -V --capabilities --config --format --verbose --quiet --no-color --timeout --robot --shadow --dry-run --standalone --fields --compact --max-tokens --estimate-tokens --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --capabilities)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "json toon md jsonl summary metrics slack exitcode prose" -- "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fields)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-tokens)
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
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _pt-core -o nosort -o bashdefault -o default pt-core
else
    complete -F _pt-core -o bashdefault -o default pt-core
fi
