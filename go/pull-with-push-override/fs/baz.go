/*
 *  Copyright © 2022 Paulo Villela. All rights reserved.
 *  Use of this source code is governed by the MIT license
 *  that can be found in the LICENSE file.
 */

package fs

import (
	"fmt"
	"github.com/pvillela/module-config/go/pull-with-push-override/config"
)

type BazCfgInfo struct {
	X string
}

type BazCfgSrc = config.CfgSrc[BazCfgInfo]

var BazCfgSrcV = config.MakeCfgSrc[BazCfgInfo](nil)

func Baz() {
	fmt.Println(len(BazCfgSrcV().X))
}
