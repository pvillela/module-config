/*
 *  Copyright © 2021 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the Apache 2.0 license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
	"github.com/pvillela/module-config/go/module-config/pull-with-push-override/config"
)

type BazCfgInfo struct {
	X string
}

type BazCfgSrc = config.CfgSrc[BazCfgInfo]

var BazCfgSrcV = config.MakeCfgSrc[BazCfgInfo](nil)

func Baz() {
	fmt.Println(len(BazCfgSrcV().X))
}
