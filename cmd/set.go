/*
Copyright © 2019 MICHAEL McDERMOTT

*/
package cmd

import (
	"github.com/spf13/cobra"
	"github.com/xkortex/kv/util"
	"github.com/xkortex/vprint"
	"strings"
)

// setCmd represents the set command
var setCmd = &cobra.Command{
	Use:     "set",
	Aliases: []string{"s"},

	Short: "Set a key to a value",
	Long: `Set a key to a value according to either command line input or 
standard in`,
	Run: func(cmd *cobra.Command, args []string) {
		stdin_struct, err := util.Get_stdin()
		vprint.Println(args, stdin_struct.Stdin)
		util.Panic_if(err)
		if len(args) == 0 {
			panic("/\\--/\\ Must have at least one argument (handling under construction)")
		}
		key := args[0]

		val := ""
		if stdin_struct.Has_stdin {
			val = stdin_struct.Stdin
		} else if len(args) > 1 {
			val = strings.Join(args[1:], " ")
		} else {
			panic("/\\--/\\ Must have at least two arguments (handling under construction)")
		}
		ns, _ := cmd.Flags().GetString("namespace")

		lookup_path := util.GetLookupPath(ns, args[0])
		vprint.Println(lookup_path)
		vprint.Println(key)
		vprint.Println("|" + val + "|")
		util.Store_value(lookup_path, key, val)

	},
}

func init() {
	RootCmd.AddCommand(setCmd)

}
