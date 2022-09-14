/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package main

import (
	"github.com/pvillela/module-config/go/arch/util"
	"github.com/pvillela/module-config/go/module-config/push-to-function/config"
	"github.com/pvillela/module-config/go/module-config/push-to-function/fs"
	"github.com/pvillela/module-config/go/module-config/push-to-function/fs/boot"
)

func main() {
	boot.FooSflCfgAdapter = func(src config.AppCfgSrc) func() fs.FooSflCfgInfo {
		return util.ThunkOf(fs.FooSflCfgInfo{X: "foo"})
	}

	boot.BarBfCfgAdapter = func(src config.AppCfgSrc) func() fs.BarBfCfgInfo {
		return util.ThunkOf(fs.BarBfCfgInfo{Z: 99})
	}

	{
		fooSfl := boot.FooSflBoot(nil)
		fooSfl()
	}

	{
		fooSfl := boot.FooSflBoot(config.GetAppConfiguration)
		fooSfl()
	}
}
