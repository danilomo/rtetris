pub type Pattern = [[bool; 4]; 4];

const ___: bool = false;
const _X_: bool = true;

pub const I_BLOCK_PATTERNS: [Pattern; 2] = [
    [
        [___, _X_, ___, ___],
        [___, _X_, ___, ___],
        [___, _X_, ___, ___],
        [___, _X_, ___, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [_X_, _X_, _X_, _X_],
    ],
];

pub const O_BLOCK_PATTERNS: [Pattern; 1] = [[
    [___, ___, ___, ___],
    [___, ___, ___, ___],
    [___, _X_, _X_, ___],
    [___, _X_, _X_, ___],
]];

pub const L_BLOCK_PATTERNS: [Pattern; 4] = [
    [
        [___, ___, ___, ___],
        [___, _X_, ___, ___],
        [___, _X_, ___, ___],
        [___, _X_, _X_, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [___, _X_, _X_, _X_],
        [___, _X_, ___, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, _X_, _X_, ___],
        [___, ___, _X_, ___],
        [___, ___, _X_, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [___, ___, ___, _X_],
        [___, _X_, _X_, _X_],
    ],
];

pub const J_BLOCK_PATTERNS: [Pattern; 4] = [
    [
        [___, ___, ___, ___],
        [___, ___, _X_, ___],
        [___, ___, _X_, ___],
        [___, _X_, _X_, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [_X_, _X_, _X_, ___],
        [___, ___, _X_, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, _X_, _X_, ___],
        [___, _X_, ___, ___],
        [___, _X_, ___, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [_X_, ___, ___, ___],
        [_X_, _X_, _X_, ___],
    ],
];

pub const T_BLOCK_PATTERNS: [Pattern; 4] = [
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [___, ___, _X_, ___],
        [___, _X_, _X_, _X_],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, _X_, ___],
        [___, _X_, _X_, ___],
        [___, ___, _X_, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, _X_, _X_, _X_],
        [___, ___, _X_, ___],
        [___, ___, ___, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, _X_, ___, ___],
        [___, _X_, _X_, ___],
        [___, _X_, ___, ___],
    ],
];

pub const S_BLOCK_PATTERNS: [Pattern; 2] = [
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [___, _X_, _X_, ___],
        [_X_, _X_, ___, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, _X_, ___, ___],
        [___, _X_, _X_, ___],
        [___, ___, _X_, ___],
    ],
];

pub const Z_BLOCK_PATTERNS: [Pattern; 2] = [
    [
        [___, ___, ___, ___],
        [___, ___, ___, ___],
        [_X_, _X_, ___, ___],
        [___, _X_, _X_, ___],
    ],
    [
        [___, ___, ___, ___],
        [___, ___, _X_, ___],
        [___, _X_, _X_, ___],
        [___, _X_, ___, ___],
    ],
];
