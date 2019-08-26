
target/xtensa-esp32-none-elf/debug/examples/test:     file format elf32-xtensa-le


Disassembly of section .literal.literal:

40080400 <.literal.literal>:
40080400:	0806d0        	lsx	f0, a6, a13
40080403:	076440        	lsi	f4, a4, 28
40080406:	4008      	l32i.n	a0, a0, 16
40080408:	080674        	lsi	f7, a6, 32
4008040b:	046040        	extui	a6, a4, 0, 1
4008040e:	4008      	l32i.n	a0, a0, 16
40080410:	0518      	l32i.n	a1, a5, 0
40080412:	4008      	l32i.n	a0, a0, 16
40080414:	080524        	lsi	f2, a5, 32
40080417:	053840        	extui	a3, a4, 24, 1
4008041a:	4008      	l32i.n	a0, a0, 16
4008041c:	05ac      	beqz.n	a5, 40080440 <_edata+0x90440>
4008041e:	4008      	l32i.n	a0, a0, 16
40080420:	05c8      	l32i.n	a12, a5, 0
40080422:	4008      	l32i.n	a0, a0, 16
40080424:	05d8      	l32i.n	a13, a5, 0
40080426:	4008      	l32i.n	a0, a0, 16
40080428:	05ec      	bnez.n	a5, 4008044c <_edata+0x9044c>
4008042a:	4008      	l32i.n	a0, a0, 16
4008042c:	0806b4        	lsi	f11, a6, 32
4008042f:	000040        	lsi	f4, a0, 0
40080432:	ff          	.byte 0xff
40080433:	3f          	.byte 0x3f
40080434:	0804d4        	lsi	f13, a4, 32
40080437:	000040        	lsi	f4, a0, 0
4008043a:	ff          	.byte 0xff
4008043b:	3f          	.byte 0x3f
4008043c:	078c      	beqz.n	a7, 40080440 <_edata+0x90440>
4008043e:	4008      	l32i.n	a0, a0, 16
40080440:	054c      	movi.n	a5, 64
40080442:	4008      	l32i.n	a0, a0, 16
40080444:	04bc      	beqz.n	a4, 40080478 <_ZN4core4sync6atomic14compiler_fence17hfb2436d62e5d7f28E+0x18>
40080446:	4008      	l32i.n	a0, a0, 16
40080448:	077c      	movi.n	a7, -16
4008044a:	4008      	l32i.n	a0, a0, 16
4008044c:	077c      	movi.n	a7, -16
4008044e:	4008      	l32i.n	a0, a0, 16
40080450:	0804a4        	lsi	f10, a4, 32
40080453:	6ab940        	maddn.s	f11, f9, f4
40080456:	497525        	call8	400c9ba8 <_iram_text_end+0x4941c>
40080459:	dfcb      	addi.n	a13, a15, 12
4008045b:	be          	.byte 0xbe
4008045c:	080630        	lsx	f0, a6, a3
4008045f:	40          	.byte 0x40

Disassembly of section .text._ZN4core4sync6atomic14compiler_fence17hfb2436d62e5d7f28E:

40080460 <_ZN4core4sync6atomic14compiler_fence17hfb2436d62e5d7f28E>:
40080460:	006136        	entry	a1, 48
40080463:	028d      	mov.n	a8, a2
40080465:	084122        	s8i	a2, a1, 8
40080468:	080122        	l8ui	a2, a1, 8
4008046b:	490c      	movi.n	a9, 4
4008046d:	1189      	s32i.n	a8, a1, 4
4008046f:	0129      	s32i.n	a2, a1, 0
40080471:	0c3927        	bltu	a9, a2, 40080481 <LBB0_2>
40080474:	ffe381        	l32r	a8, 40080400 <_edata+0x90400>
40080477:	0198      	l32i.n	a9, a1, 0
40080479:	a08980        	addx4	a8, a9, a8
4008047c:	0888      	l32i.n	a8, a8, 0
4008047e:	0008a0        	jx	a8

40080481 <LBB0_2>:
40080481:	0020c0        	memw
40080484:	000686        	j	400804a2 <LBB0_8>

40080487 <LBB0_4>:
40080487:	0020c0        	memw
4008048a:	000506        	j	400804a2 <LBB0_8>

4008048d <LBB0_5>:
4008048d:	0020c0        	memw
40080490:	000386        	j	400804a2 <LBB0_8>

40080493 <LBB0_6>:
40080493:	0020c0        	memw
40080496:	000206        	j	400804a2 <LBB0_8>

40080499 <LBB0_7>:
40080499:	ffdaa1        	l32r	a10, 40080404 <_edata+0x90404>
4008049c:	ffdb81        	l32r	a8, 40080408 <_edata+0x90408>
4008049f:	0008e0        	callx8	a8

400804a2 <LBB0_8>:
400804a2:	f01d      	retw.n

Disassembly of section .text.rust_begin_unwind:

400804a4 <rust_begin_unwind>:
400804a4:	005136        	entry	a1, 40
400804a7:	ffffc6        	j	400804aa <LBB1_1>

400804aa <LBB1_1>:
400804aa:	480c      	movi.n	a8, 4
400804ac:	004182        	s8i	a8, a1, 0
400804af:	0001a2        	l8ui	a10, a1, 0
400804b2:	ffd681        	l32r	a8, 4008040c <_edata+0x9040c>
400804b5:	0008e0        	callx8	a8
400804b8:	fffb86        	j	400804aa <LBB1_1>

Disassembly of section .text.main:

400804bc <main>:
400804bc:	005136        	entry	a1, 40
400804bf:	ffffc6        	j	400804c2 <LBB2_1>

400804c2 <LBB2_1>:
400804c2:	480c      	movi.n	a8, 4
400804c4:	004182        	s8i	a8, a1, 0
400804c7:	0001a2        	l8ui	a10, a1, 0
400804ca:	ffd081        	l32r	a8, 4008040c <_edata+0x9040c>
400804cd:	0008e0        	callx8	a8
400804d0:	fffb86        	j	400804c2 <LBB2_1>

Disassembly of section .text._ZN2r08zero_bss17h096fc00ee9f70180E:

400804d4 <_ZN2r08zero_bss17h096fc00ee9f70180E>:
400804d4:	007136        	entry	a1, 56
400804d7:	028d      	mov.n	a8, a2
400804d9:	4129      	s32i.n	a2, a1, 16
400804db:	3189      	s32i.n	a8, a1, 12
400804dd:	2139      	s32i.n	a3, a1, 8
400804df:	ffffc6        	j	400804e2 <LBB3_1>

400804e2 <LBB3_1>:
400804e2:	4188      	l32i.n	a8, a1, 16
400804e4:	2198      	l32i.n	a9, a1, 8
400804e6:	043897        	bltu	a8, a9, 400804ee <LBB3_3>
400804e9:	ffffc6        	j	400804ec <LBB3_2>

400804ec <LBB3_2>:
400804ec:	f01d      	retw.n

400804ee <LBB3_3>:
400804ee:	41a8      	l32i.n	a10, a1, 16
400804f0:	ffc881        	l32r	a8, 40080410 <_edata+0x90410>
400804f3:	11a9      	s32i.n	a10, a1, 4
400804f5:	0008e0        	callx8	a8
400804f8:	ffc781        	l32r	a8, 40080414 <_edata+0x90414>
400804fb:	1198      	l32i.n	a9, a1, 4
400804fd:	01a9      	s32i.n	a10, a1, 0
400804ff:	09ad      	mov.n	a10, a9
40080501:	01b8      	l32i.n	a11, a1, 0
40080503:	0008e0        	callx8	a8
40080506:	41a8      	l32i.n	a10, a1, 16
40080508:	1b0c      	movi.n	a11, 1
4008050a:	ffc381        	l32r	a8, 40080418 <_edata+0x90418>
4008050d:	0008e0        	callx8	a8
40080510:	41a9      	s32i.n	a10, a1, 16
40080512:	fff306        	j	400804e2 <LBB3_1>

Disassembly of section .text._ZN4core3mem6zeroed17h4b13e321b354d93dE:

40080518 <_ZN4core3mem6zeroed17h4b13e321b354d93dE>:
40080518:	005136        	entry	a1, 40
4008051b:	080c      	movi.n	a8, 0
4008051d:	0189      	s32i.n	a8, a1, 0
4008051f:	0128      	l32i.n	a2, a1, 0
40080521:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr14write_volatile17h1618492e1d4a3110E:

40080524 <_ZN4core3ptr14write_volatile17h1618492e1d4a3110E>:
40080524:	005136        	entry	a1, 40
40080527:	038d      	mov.n	a8, a3
40080529:	029d      	mov.n	a9, a2
4008052b:	0020c0        	memw
4008052e:	0239      	s32i.n	a3, a2, 0
40080530:	1199      	s32i.n	a9, a1, 4
40080532:	0189      	s32i.n	a8, a1, 0
40080534:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h508ee9a243763976E:

40080538 <_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h508ee9a243763976E>:
40080538:	006136        	entry	a1, 48
4008053b:	038d      	mov.n	a8, a3
4008053d:	029d      	mov.n	a9, a2
4008053f:	a03320        	addx4	a3, a3, a2
40080542:	2139      	s32i.n	a3, a1, 8
40080544:	2128      	l32i.n	a2, a1, 8
40080546:	1199      	s32i.n	a9, a1, 4
40080548:	0189      	s32i.n	a8, a1, 0
4008054a:	f01d      	retw.n

Disassembly of section .text._ZN2r09init_data17h594c49d33e4a83dfE:

4008054c <_ZN2r09init_data17h594c49d33e4a83dfE>:
4008054c:	008136        	entry	a1, 64
4008054f:	048d      	mov.n	a8, a4
40080551:	029d      	mov.n	a9, a2
40080553:	6129      	s32i.n	a2, a1, 24
40080555:	7149      	s32i.n	a4, a1, 28
40080557:	5139      	s32i.n	a3, a1, 20
40080559:	4189      	s32i.n	a8, a1, 16
4008055b:	3199      	s32i.n	a9, a1, 12
4008055d:	000046        	j	40080562 <LBB7_1>
	...

40080562 <LBB7_1>:
40080562:	6188      	l32i.n	a8, a1, 24
40080564:	5198      	l32i.n	a9, a1, 20
40080566:	043897        	bltu	a8, a9, 4008056e <LBB7_3>
40080569:	ffffc6        	j	4008056c <LBB7_2>

4008056c <LBB7_2>:
4008056c:	f01d      	retw.n

4008056e <LBB7_3>:
4008056e:	61a8      	l32i.n	a10, a1, 24
40080570:	7188      	l32i.n	a8, a1, 28
40080572:	ffaa91        	l32r	a9, 4008041c <_edata+0x9041c>
40080575:	21a9      	s32i.n	a10, a1, 8
40080577:	08ad      	mov.n	a10, a8
40080579:	0009e0        	callx8	a9
4008057c:	ffa981        	l32r	a8, 40080420 <_edata+0x90420>
4008057f:	2198      	l32i.n	a9, a1, 8
40080581:	11a9      	s32i.n	a10, a1, 4
40080583:	09ad      	mov.n	a10, a9
40080585:	11b8      	l32i.n	a11, a1, 4
40080587:	0008e0        	callx8	a8
4008058a:	61a8      	l32i.n	a10, a1, 24
4008058c:	180c      	movi.n	a8, 1
4008058e:	ffa291        	l32r	a9, 40080418 <_edata+0x90418>
40080591:	08bd      	mov.n	a11, a8
40080593:	0189      	s32i.n	a8, a1, 0
40080595:	0009e0        	callx8	a9
40080598:	61a9      	s32i.n	a10, a1, 24
4008059a:	71a8      	l32i.n	a10, a1, 28
4008059c:	ffa281        	l32r	a8, 40080424 <_edata+0x90424>
4008059f:	01b8      	l32i.n	a11, a1, 0
400805a1:	0008e0        	callx8	a8
400805a4:	71a9      	s32i.n	a10, a1, 28
400805a6:	ffee06        	j	40080562 <LBB7_1>

Disassembly of section .text._ZN4core3ptr4read17h89141c607774d64eE:

400805ac <_ZN4core3ptr4read17h89141c607774d64eE>:
400805ac:	006136        	entry	a1, 48
400805af:	028d      	mov.n	a8, a2
400805b1:	2198      	l32i.n	a9, a1, 8
400805b3:	1199      	s32i.n	a9, a1, 4
400805b5:	b14b      	addi.n	a11, a1, 4
400805b7:	1c0c      	movi.n	a12, 1
400805b9:	ff9b91        	l32r	a9, 40080428 <_edata+0x90428>
400805bc:	02ad      	mov.n	a10, a2
400805be:	006182        	s32i	a8, a1, 0
400805c1:	0009e0        	callx8	a9
400805c4:	1128      	l32i.n	a2, a1, 4
400805c6:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr5write17hf28647bd0572798eE:

400805c8 <_ZN4core3ptr5write17hf28647bd0572798eE>:
400805c8:	005136        	entry	a1, 40
400805cb:	038d      	mov.n	a8, a3
400805cd:	029d      	mov.n	a9, a2
400805cf:	0239      	s32i.n	a3, a2, 0
400805d1:	1199      	s32i.n	a9, a1, 4
400805d3:	0189      	s32i.n	a8, a1, 0
400805d5:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17hbba87d79337edb5dE:

400805d8 <_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17hbba87d79337edb5dE>:
400805d8:	006136        	entry	a1, 48
400805db:	038d      	mov.n	a8, a3
400805dd:	029d      	mov.n	a9, a2
400805df:	a03320        	addx4	a3, a3, a2
400805e2:	2139      	s32i.n	a3, a1, 8
400805e4:	2128      	l32i.n	a2, a1, 8
400805e6:	1199      	s32i.n	a9, a1, 4
400805e8:	0189      	s32i.n	a8, a1, 0
400805ea:	f01d      	retw.n

Disassembly of section .text._ZN4core10intrinsics19copy_nonoverlapping17h406b1b058c69b7d4E:

400805ec <_ZN4core10intrinsics19copy_nonoverlapping17h406b1b058c69b7d4E>:
400805ec:	006136        	entry	a1, 48
400805ef:	048d      	mov.n	a8, a4
400805f1:	039d      	mov.n	a9, a3
400805f3:	02ad      	mov.n	a10, a2
400805f5:	11c4e0        	slli	a12, a4, 2
400805f8:	ff8d41        	l32r	a4, 4008042c <_edata+0x9042c>
400805fb:	31a9      	s32i.n	a10, a1, 12
400805fd:	03ad      	mov.n	a10, a3
400805ff:	02bd      	mov.n	a11, a2
40080601:	2189      	s32i.n	a8, a1, 8
40080603:	1199      	s32i.n	a9, a1, 4
40080605:	0004e0        	callx8	a4
40080608:	01a9      	s32i.n	a10, a1, 0
4008060a:	f01d      	retw.n

Disassembly of section .text.Reset:

4008060c <Reset>:
4008060c:	004136        	entry	a1, 32
4008060f:	ff88a1        	l32r	a10, 40080430 <_edata+0x90430>
40080612:	ff87b1        	l32r	a11, 40080430 <_edata+0x90430>
40080615:	ff8781        	l32r	a8, 40080434 <_edata+0x90434>
40080618:	0008e0        	callx8	a8
4008061b:	ff87a1        	l32r	a10, 40080438 <_edata+0x90438>
4008061e:	ff86b1        	l32r	a11, 40080438 <_edata+0x90438>
40080621:	ff86c1        	l32r	a12, 4008043c <_edata+0x9043c>
40080624:	ff8781        	l32r	a8, 40080440 <_edata+0x90440>
40080627:	0008e0        	callx8	a8
4008062a:	ff8681        	l32r	a8, 40080444 <_edata+0x90444>
4008062d:	0008e0        	callx8	a8

Disassembly of section .text._ZN4core9panicking9panic_fmt17hebe7856dd84b9e42E:

40080630 <_ZN4core9panicking9panic_fmt17hebe7856dd84b9e42E>:
40080630:	009136        	entry	a1, 72
40080633:	038d      	mov.n	a8, a3
40080635:	029d      	mov.n	a9, a2
40080637:	03a8      	l32i.n	a10, a3, 0
40080639:	13b8      	l32i.n	a11, a3, 4
4008063b:	23c8      	l32i.n	a12, a3, 8
4008063d:	3338      	l32i.n	a3, a3, 12
4008063f:	ff82d1        	l32r	a13, 40080448 <_edata+0x90448>
40080642:	21d9      	s32i.n	a13, a1, 8
40080644:	ff82d1        	l32r	a13, 4008044c <_edata+0x9044c>
40080647:	31d9      	s32i.n	a13, a1, 12
40080649:	4129      	s32i.n	a2, a1, 16
4008064b:	51a9      	s32i.n	a10, a1, 20
4008064d:	61b9      	s32i.n	a11, a1, 24
4008064f:	71c9      	s32i.n	a12, a1, 28
40080651:	8139      	s32i.n	a3, a1, 32
40080653:	a18b      	addi.n	a10, a1, 8
40080655:	ff7eb1        	l32r	a11, 40080450 <_edata+0x90450>
40080658:	1199      	s32i.n	a9, a1, 4
4008065a:	006182        	s32i	a8, a1, 0
4008065d:	000be0        	callx8	a11

Disassembly of section .text._ZN4core3ptr18real_drop_in_place17h158e3df191db6aa4E:

40080660 <_ZN4core3ptr18real_drop_in_place17h158e3df191db6aa4E>:
40080660:	004136        	entry	a1, 32
40080663:	f01d      	retw.n

Disassembly of section .text._ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc0dea5d801bb051cE:

40080668 <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc0dea5d801bb051cE>:
40080668:	004136        	entry	a1, 32
4008066b:	ff7a21        	l32r	a2, 40080454 <_edata+0x90454>
4008066e:	ff7a31        	l32r	a3, 40080458 <_edata+0x90458>
40080671:	f01d      	retw.n

Disassembly of section .text._ZN4core9panicking5panic17h90a95b803104174dE:

40080674 <_ZN4core9panicking5panic17h90a95b803104174dE>:
40080674:	00b136        	entry	a1, 88
40080677:	028d      	mov.n	a8, a2
40080679:	0298      	l32i.n	a9, a2, 0
4008067b:	12a8      	l32i.n	a10, a2, 4
4008067d:	22b8      	l32i.n	a11, a2, 8
4008067f:	32c8      	l32i.n	a12, a2, 12
40080681:	42d8      	l32i.n	a13, a2, 16
40080683:	5228      	l32i.n	a2, a2, 20
40080685:	7199      	s32i.n	a9, a1, 28
40080687:	81a9      	s32i.n	a10, a1, 32
40080689:	1cc192        	addi	a9, a1, 28
4008068c:	1199      	s32i.n	a9, a1, 4
4008068e:	190c      	movi.n	a9, 1
40080690:	2199      	s32i.n	a9, a1, 8
40080692:	090c      	movi.n	a9, 0
40080694:	3199      	s32i.n	a9, a1, 12
40080696:	ff6ca1        	l32r	a10, 40080448 <_edata+0x90448>
40080699:	51a9      	s32i.n	a10, a1, 20
4008069b:	6199      	s32i.n	a9, a1, 24
4008069d:	91b9      	s32i.n	a11, a1, 36
4008069f:	a1c9      	s32i.n	a12, a1, 40
400806a1:	b1d9      	s32i.n	a13, a1, 44
400806a3:	c129      	s32i.n	a2, a1, 48
400806a5:	04c1a2        	addi	a10, a1, 4
400806a8:	24c1b2        	addi	a11, a1, 36
400806ab:	ff6c91        	l32r	a9, 4008045c <_edata+0x9045c>
400806ae:	006182        	s32i	a8, a1, 0
400806b1:	0009e0        	callx8	a9

Disassembly of section .text.memcpy:

400806b4 <memcpy>:
400806b4:	004136        	entry	a1, 32
400806b7:	011416        	beqz	a4, 400806cc <LBB0_3>
400806ba:	00a082        	movi	a8, 0

400806bd <LBB0_2>:
400806bd:	928a      	add.n	a9, a2, a8
400806bf:	a38a      	add.n	a10, a3, a8
400806c1:	000aa2        	l8ui	a10, a10, 0
400806c4:	0049a2        	s8i	a10, a9, 0
400806c7:	881b      	addi.n	a8, a8, 1
400806c9:	f09847        	bne	a8, a4, 400806bd <LBB0_2>

400806cc <LBB0_3>:
400806cc:	f01d      	retw.n
