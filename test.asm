
target/xtensa-esp32-none-elf/debug/examples/test:     file format elf32-xtensa-le


Disassembly of section .literal.literal:

40080400 <.literal.literal>:
40080400:	0488      	l32i.n	a8, a4, 0
40080402:	4008      	l32i.n	a0, a0, 16
40080404:	080494        	lsi	f9, a4, 32
40080407:	04a840        	extui	a10, a4, 8, 1
4008040a:	4008      	l32i.n	a0, a0, 16
4008040c:	051c      	movi.n	a5, 16
4008040e:	4008      	l32i.n	a0, a0, 16
40080410:	0538      	l32i.n	a3, a5, 0
40080412:	4008      	l32i.n	a0, a0, 16
40080414:	0548      	l32i.n	a4, a5, 0
40080416:	4008      	l32i.n	a0, a0, 16
40080418:	055c      	movi.n	a5, 80
4008041a:	4008      	l32i.n	a0, a0, 16
4008041c:	0805a0        	lsx	f0, a5, a10
4008041f:	000040        	lsi	f4, a0, 0
40080422:	ff          	.byte 0xff
40080423:	3f          	.byte 0x3f
40080424:	080444        	mula.dd.ll.ldinc	m0, a4, m0, m3
40080427:	000040        	lsi	f4, a0, 0
4008042a:	ff          	.byte 0xff
4008042b:	3f          	.byte 0x3f
4008042c:	05ba      	add.n	a0, a5, a11
4008042e:	4008      	l32i.n	a0, a0, 16
40080430:	04bc      	beqz.n	a4, 40080464 <LBB1_3+0x6>
40080432:	4008      	l32i.n	a0, a0, 16
40080434:	0438      	l32i.n	a3, a4, 0
40080436:	4008      	l32i.n	a0, a0, 16

Disassembly of section .text.main:

40080438 <main>:
40080438:	004136        	entry	a1, 32
4008043b:	ffffc6        	j	4008043e <LBB0_1>

4008043e <LBB0_1>:
4008043e:	ffff06        	j	4008043e <LBB0_1>

Disassembly of section .text._ZN2r08zero_bss17h0b0e47f405357188E:

40080444 <_ZN2r08zero_bss17h0b0e47f405357188E>:
40080444:	007136        	entry	a1, 56
40080447:	028d      	mov.n	a8, a2
40080449:	4129      	s32i.n	a2, a1, 16
4008044b:	3189      	s32i.n	a8, a1, 12
4008044d:	2139      	s32i.n	a3, a1, 8
4008044f:	ffffc6        	j	40080452 <LBB1_1>

40080452 <LBB1_1>:
40080452:	4188      	l32i.n	a8, a1, 16
40080454:	2198      	l32i.n	a9, a1, 8
40080456:	043897        	bltu	a8, a9, 4008045e <LBB1_3>
40080459:	ffffc6        	j	4008045c <LBB1_2>

4008045c <LBB1_2>:
4008045c:	f01d      	retw.n

4008045e <LBB1_3>:
4008045e:	41a8      	l32i.n	a10, a1, 16
40080460:	ffe881        	l32r	a8, 40080400 <_edata+0x90400>
40080463:	11a9      	s32i.n	a10, a1, 4
40080465:	0008e0        	callx8	a8
40080468:	ffe781        	l32r	a8, 40080404 <_edata+0x90404>
4008046b:	1198      	l32i.n	a9, a1, 4
4008046d:	01a9      	s32i.n	a10, a1, 0
4008046f:	09ad      	mov.n	a10, a9
40080471:	01b8      	l32i.n	a11, a1, 0
40080473:	0008e0        	callx8	a8
40080476:	41a8      	l32i.n	a10, a1, 16
40080478:	1b0c      	movi.n	a11, 1
4008047a:	ffe381        	l32r	a8, 40080408 <_edata+0x90408>
4008047d:	0008e0        	callx8	a8
40080480:	41a9      	s32i.n	a10, a1, 16
40080482:	fff306        	j	40080452 <LBB1_1>

Disassembly of section .text._ZN4core3mem6zeroed17hfe572f815ac752f6E:

40080488 <_ZN4core3mem6zeroed17hfe572f815ac752f6E>:
40080488:	005136        	entry	a1, 40
4008048b:	080c      	movi.n	a8, 0
4008048d:	0189      	s32i.n	a8, a1, 0
4008048f:	0128      	l32i.n	a2, a1, 0
40080491:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr14write_volatile17hf6dd4a44d4ba61f9E:

40080494 <_ZN4core3ptr14write_volatile17hf6dd4a44d4ba61f9E>:
40080494:	005136        	entry	a1, 40
40080497:	038d      	mov.n	a8, a3
40080499:	029d      	mov.n	a9, a2
4008049b:	0020c0        	memw
4008049e:	0239      	s32i.n	a3, a2, 0
400804a0:	1199      	s32i.n	a9, a1, 4
400804a2:	0189      	s32i.n	a8, a1, 0
400804a4:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h8e7754978ce5a9eeE:

400804a8 <_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h8e7754978ce5a9eeE>:
400804a8:	006136        	entry	a1, 48
400804ab:	038d      	mov.n	a8, a3
400804ad:	029d      	mov.n	a9, a2
400804af:	a03320        	addx4	a3, a3, a2
400804b2:	2139      	s32i.n	a3, a1, 8
400804b4:	2128      	l32i.n	a2, a1, 8
400804b6:	1199      	s32i.n	a9, a1, 4
400804b8:	0189      	s32i.n	a8, a1, 0
400804ba:	f01d      	retw.n

Disassembly of section .text._ZN2r09init_data17h7f54638af56ad387E:

400804bc <_ZN2r09init_data17h7f54638af56ad387E>:
400804bc:	008136        	entry	a1, 64
400804bf:	048d      	mov.n	a8, a4
400804c1:	029d      	mov.n	a9, a2
400804c3:	6129      	s32i.n	a2, a1, 24
400804c5:	7149      	s32i.n	a4, a1, 28
400804c7:	5139      	s32i.n	a3, a1, 20
400804c9:	4189      	s32i.n	a8, a1, 16
400804cb:	3199      	s32i.n	a9, a1, 12
400804cd:	ffffc6        	j	400804d0 <LBB5_1>

400804d0 <LBB5_1>:
400804d0:	6188      	l32i.n	a8, a1, 24
400804d2:	5198      	l32i.n	a9, a1, 20
400804d4:	063897        	bltu	a8, a9, 400804de <LBB5_3>
400804d7:	ffffc6        	j	400804da <LBB5_2>

400804da <LBB5_2>:
400804da:	f01d      	retw.n
	...

400804de <LBB5_3>:
400804de:	61a8      	l32i.n	a10, a1, 24
400804e0:	7188      	l32i.n	a8, a1, 28
400804e2:	ffca91        	l32r	a9, 4008040c <_edata+0x9040c>
400804e5:	21a9      	s32i.n	a10, a1, 8
400804e7:	08ad      	mov.n	a10, a8
400804e9:	0009e0        	callx8	a9
400804ec:	ffc981        	l32r	a8, 40080410 <_edata+0x90410>
400804ef:	2198      	l32i.n	a9, a1, 8
400804f1:	11a9      	s32i.n	a10, a1, 4
400804f3:	09ad      	mov.n	a10, a9
400804f5:	11b8      	l32i.n	a11, a1, 4
400804f7:	0008e0        	callx8	a8
400804fa:	61a8      	l32i.n	a10, a1, 24
400804fc:	180c      	movi.n	a8, 1
400804fe:	ffc291        	l32r	a9, 40080408 <_edata+0x90408>
40080501:	08bd      	mov.n	a11, a8
40080503:	0189      	s32i.n	a8, a1, 0
40080505:	0009e0        	callx8	a9
40080508:	61a9      	s32i.n	a10, a1, 24
4008050a:	71a8      	l32i.n	a10, a1, 28
4008050c:	ffc281        	l32r	a8, 40080414 <_edata+0x90414>
4008050f:	01b8      	l32i.n	a11, a1, 0
40080511:	0008e0        	callx8	a8
40080514:	71a9      	s32i.n	a10, a1, 28
40080516:	ffed86        	j	400804d0 <LBB5_1>

Disassembly of section .text._ZN4core3ptr4read17h0b6fc24fe412d226E:

4008051c <_ZN4core3ptr4read17h0b6fc24fe412d226E>:
4008051c:	006136        	entry	a1, 48
4008051f:	028d      	mov.n	a8, a2
40080521:	2198      	l32i.n	a9, a1, 8
40080523:	1199      	s32i.n	a9, a1, 4
40080525:	b14b      	addi.n	a11, a1, 4
40080527:	1c0c      	movi.n	a12, 1
40080529:	ffbb91        	l32r	a9, 40080418 <_edata+0x90418>
4008052c:	02ad      	mov.n	a10, a2
4008052e:	006182        	s32i	a8, a1, 0
40080531:	0009e0        	callx8	a9
40080534:	1128      	l32i.n	a2, a1, 4
40080536:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr5write17h144e756bc7bdc76cE:

40080538 <_ZN4core3ptr5write17h144e756bc7bdc76cE>:
40080538:	005136        	entry	a1, 40
4008053b:	038d      	mov.n	a8, a3
4008053d:	029d      	mov.n	a9, a2
4008053f:	0239      	s32i.n	a3, a2, 0
40080541:	1199      	s32i.n	a9, a1, 4
40080543:	0189      	s32i.n	a8, a1, 0
40080545:	f01d      	retw.n

Disassembly of section .text._ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h4201c1cb528047ddE:

40080548 <_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h4201c1cb528047ddE>:
40080548:	006136        	entry	a1, 48
4008054b:	038d      	mov.n	a8, a3
4008054d:	029d      	mov.n	a9, a2
4008054f:	a03320        	addx4	a3, a3, a2
40080552:	2139      	s32i.n	a3, a1, 8
40080554:	2128      	l32i.n	a2, a1, 8
40080556:	1199      	s32i.n	a9, a1, 4
40080558:	0189      	s32i.n	a8, a1, 0
4008055a:	f01d      	retw.n

Disassembly of section .text._ZN4core10intrinsics19copy_nonoverlapping17h5a9062b38e072b78E:

4008055c <_ZN4core10intrinsics19copy_nonoverlapping17h5a9062b38e072b78E>:
4008055c:	006136        	entry	a1, 48
4008055f:	048d      	mov.n	a8, a4
40080561:	039d      	mov.n	a9, a3
40080563:	02ad      	mov.n	a10, a2
40080565:	11c4e0        	slli	a12, a4, 2
40080568:	ffad41        	l32r	a4, 4008041c <_edata+0x9041c>
4008056b:	31a9      	s32i.n	a10, a1, 12
4008056d:	03ad      	mov.n	a10, a3
4008056f:	02bd      	mov.n	a11, a2
40080571:	2189      	s32i.n	a8, a1, 8
40080573:	1199      	s32i.n	a9, a1, 4
40080575:	0004e0        	callx8	a4
40080578:	01a9      	s32i.n	a10, a1, 0
4008057a:	f01d      	retw.n

Disassembly of section .text.Reset:

4008057c <Reset>:
4008057c:	004136        	entry	a1, 32
4008057f:	ffa8a1        	l32r	a10, 40080420 <_edata+0x90420>
40080582:	ffa7b1        	l32r	a11, 40080420 <_edata+0x90420>
40080585:	ffa781        	l32r	a8, 40080424 <_edata+0x90424>
40080588:	0008e0        	callx8	a8
4008058b:	ffa7a1        	l32r	a10, 40080428 <_edata+0x90428>
4008058e:	ffa6b1        	l32r	a11, 40080428 <_edata+0x90428>
40080591:	ffa6c1        	l32r	a12, 4008042c <_edata+0x9042c>
40080594:	ffa781        	l32r	a8, 40080430 <_edata+0x90430>
40080597:	0008e0        	callx8	a8
4008059a:	ffa681        	l32r	a8, 40080434 <_edata+0x90434>
4008059d:	0008e0        	callx8	a8

Disassembly of section .text.memcpy:

400805a0 <memcpy>:
400805a0:	004136        	entry	a1, 32
400805a3:	011416        	beqz	a4, 400805b8 <LBB0_3>
400805a6:	00a082        	movi	a8, 0

400805a9 <LBB0_2>:
400805a9:	928a      	add.n	a9, a2, a8
400805ab:	a38a      	add.n	a10, a3, a8
400805ad:	000aa2        	l8ui	a10, a10, 0
400805b0:	0049a2        	s8i	a10, a9, 0
400805b3:	881b      	addi.n	a8, a8, 1
400805b5:	f09847        	bne	a8, a4, 400805a9 <LBB0_2>

400805b8 <LBB0_3>:
400805b8:	f01d      	retw.n
