let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Documents/RUST_projects/learnOpenGL/src
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +29 main.rs
badd +33 ~/documents/rust_projects/learnopengl/src/object_manager/mod.rs
badd +62 ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs
badd +52 ~/documents/rust_projects/learnopengl/src/object_manager/physics_manager/mod.rs
badd +45 ~/documents/rust_projects/learnopengl/src/object_manager/graphic_debug/mod.rs
badd +164 ~/.cargo/registry/src/github.com-1ecc6299db9ec823/winit-0.27.5/src/window.rs
badd +183 c:/ProgramData/chocolatey/lib/rust/tools/lib/rustlib/src/rust/library/core/src/default.rs
badd +108 fragment_shader.glsl
argglobal
%argdel
edit main.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 97 + 146) / 293)
exe '2resize ' . ((&lines * 36 + 40) / 80)
exe 'vert 2resize ' . ((&columns * 97 + 146) / 293)
exe '3resize ' . ((&lines * 41 + 40) / 80)
exe 'vert 3resize ' . ((&columns * 97 + 146) / 293)
exe 'vert 4resize ' . ((&columns * 97 + 146) / 293)
argglobal
balt ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 85 - ((69 * winheight(0) + 39) / 78)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 85
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("~/documents/rust_projects/learnopengl/src/object_manager/mod.rs", ":p")) | buffer ~/documents/rust_projects/learnopengl/src/object_manager/mod.rs | else | edit ~/documents/rust_projects/learnopengl/src/object_manager/mod.rs | endif
if &buftype ==# 'terminal'
  silent file ~/documents/rust_projects/learnopengl/src/object_manager/mod.rs
endif
balt ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 31 - ((24 * winheight(0) + 18) / 36)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 31
normal! 042|
wincmd w
argglobal
if bufexists(fnamemodify("~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs", ":p")) | buffer ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs | else | edit ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs | endif
if &buftype ==# 'terminal'
  silent file ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs
endif
balt ~/documents/rust_projects/learnopengl/src/object_manager/mod.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 49 - ((8 * winheight(0) + 20) / 41)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 49
let s:c = 8 - ((0 * winwidth(0) + 48) / 97)
if s:c > 0
  exe 'normal! ' . s:c . '|zs' . 8 . '|'
else
  normal! 08|
endif
wincmd w
argglobal
if bufexists(fnamemodify("~/documents/rust_projects/learnopengl/src/object_manager/physics_manager/mod.rs", ":p")) | buffer ~/documents/rust_projects/learnopengl/src/object_manager/physics_manager/mod.rs | else | edit ~/documents/rust_projects/learnopengl/src/object_manager/physics_manager/mod.rs | endif
if &buftype ==# 'terminal'
  silent file ~/documents/rust_projects/learnopengl/src/object_manager/physics_manager/mod.rs
endif
balt ~/documents/rust_projects/learnopengl/src/object_manager/object/mod.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 33 - ((30 * winheight(0) + 39) / 78)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 33
normal! 035|
wincmd w
4wincmd w
exe 'vert 1resize ' . ((&columns * 97 + 146) / 293)
exe '2resize ' . ((&lines * 36 + 40) / 80)
exe 'vert 2resize ' . ((&columns * 97 + 146) / 293)
exe '3resize ' . ((&lines * 41 + 40) / 80)
exe 'vert 3resize ' . ((&columns * 97 + 146) / 293)
exe 'vert 4resize ' . ((&columns * 97 + 146) / 293)
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
