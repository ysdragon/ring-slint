load "slint.ring"

aImages = [
    "images/photo1.jpg",
    "images/photo2.jpg",
    "images/photo3.jpg",
    "images/photo4.jpg",
    "images/photo5.jpg"
]

nCurrent = 1

oApp = new SlintApp {
    loadUI("12_images.slint")
    set("image-count", len(aImages))
    setCallback("shuffle", :onShuffle)
    setCallback("next-image", :onNext)
    setCallback("prev-image", :onPrev)
    show()
}

showImage(1)
oApp.run()

func showImage nIndex
    if nIndex < 1 nIndex = len(aImages) ok
    if nIndex > len(aImages) nIndex = 1 ok
    nCurrent = nIndex
    cPath = aImages[nCurrent]
    oApp.setImage("current-image", cPath)
    oApp.set("image-index", nCurrent - 1)
    oApp.set("image-label", cPath)
    ? "Showing: " + cPath

func onShuffle
    nRand = random(len(aImages) - 1) + 1
    if nRand = nCurrent and len(aImages) > 1
        nRand = nRand % len(aImages) + 1
    ok
    showImage(nRand)

func onNext
    showImage(nCurrent + 1)

func onPrev
    showImage(nCurrent - 1)
