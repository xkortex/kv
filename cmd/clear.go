/*
Copyright © 2019 MICHAEL McDERMOTT

*/
package cmd

import (
	"github.com/spf13/cobra"
	"github.com/xkortex/kv/util"
	"github.com/xkortex/vprint"
	"log"
	"os"
)

// clearCmd represents the clear command
var clearCmd = &cobra.Command{
	Use:   "clear",
	Short: "Clear the entire namespace",
	Long: `Removes all key-value pairs from the namespace and any children 
namespaces as well. `,
	Run: func(cmd *cobra.Command, args []string) {
		ns, _ := cmd.Flags().GetString("namespace")
		silent, _ := cmd.Flags().GetBool("silent")
		lookup_path := util.GetLookupPath(ns, "")
		vprint.Println("Clearing: ", lookup_path)
		err := os.RemoveAll(lookup_path)
		if err != nil && !silent {
			log.Fatal(err)
		}
	},
}

func init() {
	RootCmd.AddCommand(clearCmd)

}
